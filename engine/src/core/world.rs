use crate::{
    clock::Clock,
    rendering::{drawable, renderer},
};
use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    iter::Zip,
    slice::{Iter, IterMut},
};

pub trait IEntity: Copy + Clone + PartialEq + Eq + Debug {}

impl IEntity for usize {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

impl Entity {
    #[inline]
    pub fn new(id: u32, generation: u32) -> Self {
        Entity { id, generation }
    }
}

impl IEntity for Entity {}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
        state.write_u32(self.generation);
    }
}

type System = fn(&mut World, &Clock);

pub enum Scheduler {
    Startup,
    Update,
    Quit,
}

pub trait Storage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_component(&mut self, entity: &Entity) -> bool;
}

pub trait Component: 'static {
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }
}

pub trait BundleComponent: 'static {
    fn add_components_to(world: &mut World, entity: Entity, components: Self);
}

impl BundleComponent for () {
    fn add_components_to(_world: &mut World, _entity: Entity, _no_component: Self) {}
}

impl<T: Component> BundleComponent for T {
    fn add_components_to(world: &mut World, entity: Entity, component: T) {
        world.add_component_to(entity, component);
    }
}

impl<T1: Component, T2: Component> BundleComponent for (T1, T2) {
    fn add_components_to(world: &mut World, entity: Entity, components: (T1, T2)) {
        world.add_component_to(entity, components.0);
        world.add_component_to(entity, components.1);
    }
}

impl<T1: Component, T2: Component, T3: Component> BundleComponent for (T1, T2, T3) {
    fn add_components_to(world: &mut World, entity: Entity, components: (T1, T2, T3)) {
        world.add_component_to(entity, components.0);
        world.add_component_to(entity, components.1);
        world.add_component_to(entity, components.2);
    }
}

pub struct ComponentIterator<'a, T: Component> {
    iter: Zip<Iter<'a, Entity>, Iter<'a, T>>,
}

impl<'a, T: Component> ComponentIterator<'a, T> {
    fn new(storage: &'a ComponentStorage<T>) -> Self {
        Self {
            iter: storage.entities.iter().zip(storage.components.iter()),
        }
    }
}

impl<'a, T: Component> Iterator for ComponentIterator<'a, T> {
    type Item = (&'a Entity, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct ComponentIteratorMut<'a, T: Component> {
    iter: Zip<Iter<'a, Entity>, IterMut<'a, T>>,
}

impl<'a, T: Component + 'a> ComponentIteratorMut<'a, T> {
    fn new(storage: &'a mut ComponentStorage<T>) -> Self {
        // Destruct struct
        let ComponentStorage {
            alias: _,
            components,
            entities,
            free_comp: _,
        } = storage;

        Self {
            iter: entities.iter().zip(components.iter_mut()),
        }
    }
}

impl<'a, T: Component + 'a> Iterator for ComponentIteratorMut<'a, T> {
    type Item = (&'a Entity, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct ComponentStorage<T: Component> {
    alias: Vec<Option<usize>>,
    components: Vec<T>,
    entities: Vec<Entity>,
    free_comp: VecDeque<usize>,
}

impl<T: Component> ComponentStorage<T> {
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        if let Some(index) = self.alias[entity.id as usize] {
            self.components.get(index).and_then(|t| {
                if self.entities[index] == *entity {
                    Some(t)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        if let Some(index) = self.alias[entity.id as usize] {
            self.components.get_mut(index).and_then(|t| {
                if self.entities[index] == *entity {
                    Some(t)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn iter(&self) -> ComponentIterator<'_, T> {
        ComponentIterator::new(self)
    }

    fn iter_mut(&mut self) -> ComponentIteratorMut<'_, T> {
        ComponentIteratorMut::new(self)
    }
    fn add_component(&mut self, entity: Entity, component: T) {
        if self.alias.len() <= entity.id as usize {
            self.alias.resize(entity.id as usize + 1, None);
        }

        if let Some(index) = self.free_comp.pop_front() {
            self.alias[entity.id as usize] = Some(index);
            self.components[index] = component;
            self.entities[index] = entity;
        } else {
            self.alias[entity.id as usize] = Some(self.components.len());
            self.components.push(component);
            self.entities.push(entity);
        }
    }
}

impl<T: Component> Default for ComponentStorage<T> {
    fn default() -> Self {
        Self {
            alias: Vec::new(),
            components: Vec::new(),
            entities: Vec::new(),
            free_comp: VecDeque::new(),
        }
    }
}

impl<T: Component> Storage for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_component(&mut self, entity: &Entity) -> bool {
        let id = entity.id as usize;

        if id >= self.alias.iter().len() {
            return false;
        }

        if self.alias[id].is_none_or(|i| self.entities[i] != *entity) {
            return false;
        }

        // Remove the alias and push it to the free components queue
        self.free_comp.push_back(self.alias[id].take().unwrap());

        true
    }
}

#[derive(Default)]
pub struct World {
    next_id: u32,
    generations: Vec<u32>,
    free_ids: VecDeque<u32>,
    alive_count: usize,

    components: HashMap<TypeId, Box<dyn Storage>>,

    startup_systems: Vec<System>,
    update_systems: Vec<System>,
    quit_systems: Vec<System>,

    is_running: bool,
}

impl World {
    pub fn spawn_with<B: BundleComponent>(&mut self, components: B) -> Entity {
        // Generate entity
        let entity = self.spawn();
        // Add component to the entity
        B::add_components_to(self, entity, components);

        entity
    }

    pub fn spawn(&mut self) -> Entity {
        let id = if let Some(recycled) = self.free_ids.pop_front() {
            recycled
        } else {
            let fresh = self.next_id;
            self.next_id += 1;
            fresh
        };

        if (id as usize) >= self.generations.len() {
            self.generations.resize(id as usize + 1, 0);
        }

        let generation = self.generations[id as usize];
        self.alive_count += 1;

        Entity::new(id, generation)
    }

    pub fn delete(&mut self, entity: Entity) -> bool {
        let id = entity.id as usize;

        if id >= self.generations.len() {
            return false;
        }

        if self.generations[id] != entity.generation {
            return false;
        }

        // Remove each components of the entity
        for component in self.components.values_mut() {
            component.remove_component(&entity);
        }

        self.generations[id] = self.generations[id].wrapping_add(1);

        self.free_ids.push_back(entity.id);
        self.alive_count -= 1;

        true
    }

    pub fn add_system(&mut self, scheduler: Scheduler, system: System) -> &mut Self {
        match scheduler {
            Scheduler::Startup => self.startup_systems.push(system),
            Scheduler::Update => self.update_systems.push(system),
            Scheduler::Quit => self.quit_systems.push(system),
        }
        self
    }

    pub fn add_component_to<C: Component>(&mut self, entity: Entity, component: C) {
        self.components
            .entry(C::type_id())
            .or_insert(Box::new(ComponentStorage::<C>::default()))
            .as_any_mut()
            .downcast_mut::<ComponentStorage<C>>()
            .unwrap()
            .add_component(entity, component);
    }

    pub fn add_components_to<B: BundleComponent>(&mut self, entity: Entity, components: B) {
        B::add_components_to(self, entity, components);
    }

    pub fn get_entity_component<C: Component>(&self, entity: &Entity) -> Option<&C> {
        self.components.get(&C::type_id()).and_then(|value| {
            value
                .as_any()
                .downcast_ref::<ComponentStorage<C>>()
                .unwrap()
                .get(entity)
        })
    }

    pub fn get_entity_component_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        self.components.get_mut(&C::type_id()).and_then(|value| {
            value
                .as_any_mut()
                .downcast_mut::<ComponentStorage<C>>()
                .unwrap()
                .get_mut(entity)
        })
    }

    pub fn iter_component<C: Component>(&self) -> Option<ComponentIterator<'_, C>> {
        self.components.get(&C::type_id()).map(|value| {
            value
                .as_any()
                .downcast_ref::<ComponentStorage<C>>()
                .unwrap()
                .iter()
        })
    }

    pub fn iter_component_mut<C: Component>(&mut self) -> Option<ComponentIteratorMut<'_, C>> {
        self.components.get_mut(&C::type_id()).map(|value| {
            value
                .as_any_mut()
                .downcast_mut::<ComponentStorage<C>>()
                .unwrap()
                .iter_mut()
        })
    }

    pub fn get_component<C: Component>(&self) -> Option<&ComponentStorage<C>> {
        self.components
            .get(&C::type_id())
            .map(|v| v.as_any().downcast_ref::<ComponentStorage<C>>().unwrap())
    }

    #[inline]
    pub fn is_alive(&self, entity: Entity) -> bool {
        let id = entity.id as usize;

        id < self.generations.len() && self.generations[id] == entity.generation
    }

    pub fn run(&mut self) {
        self.is_running = true;
        let clock = Clock::new();
        // Run startup systems

        let startup_systems = std::mem::take(&mut self.startup_systems);

        for system in &startup_systems {
            system(self, &clock);
        }

        // Run update systems while the world is running
        let update_systems = std::mem::take(&mut self.update_systems);

        while self.is_running {
            for system in &update_systems {
                system(self, &clock);
            }
        }

        // Once the world stopped to run, run quit systems
        let quit_systems = std::mem::take(&mut self.quit_systems);

        for system in &quit_systems {
            system(self, &clock);
        }

        // Drop drawable then renderer
        self.drop_drawable();
        self.drop_renderer();
    }

    fn drop_drawable(&mut self) {
        if let Some(lines) = self
            .components
            .remove(&<drawable::Lines as Component>::type_id())
        {
            drop(lines);
        }
        if let Some(meshes) = self
            .components
            .remove(&<drawable::Mesh as Component>::type_id())
        {
            drop(meshes);
        }
    }

    fn drop_renderer(&mut self) {
        if let Some(renderer) = self
            .components
            .remove(&<renderer::Renderer as Component>::type_id())
        {
            drop(renderer);
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }
}
