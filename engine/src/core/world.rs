use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
    hash::Hash,
    iter::Zip,
    slice::{Iter, IterMut},
    thread::sleep,
    time::{Duration, Instant},
};

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

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
        state.write_u32(self.generation);
    }
}

type System = fn(&mut World);

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

pub struct World {
    next_id: u32,
    generations: Vec<u32>,
    free_ids: VecDeque<u32>,
    alive_count: usize,

    components: HashMap<TypeId, Box<dyn Storage>>,

    frame_rate_ms: f64,

    startup_systems: Vec<System>,
    update_systems: Vec<System>,
    quit_systems: Vec<System>,

    is_running: bool,
}

impl World {
    pub fn new(frame_rate_ms: f64) -> Self {
        if frame_rate_ms <= 0.0 {
            panic!("Frame rate could be equal or less than 0.0");
        }
        Self {
            next_id: 0,
            generations: Vec::new(),
            free_ids: VecDeque::new(),
            alive_count: 0,
            frame_rate_ms,
            startup_systems: Vec::new(),
            update_systems: Vec::new(),
            quit_systems: Vec::new(),
            is_running: false,
            components: HashMap::new(),
        }
    }

    pub fn spawn<C: Component>(&mut self, component: C) -> Entity {
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

        let entity = Entity::new(id, generation);

        self.components
            .entry(C::type_id())
            .or_insert(Box::new(ComponentStorage::<C>::default()))
            .as_any_mut()
            .downcast_mut::<ComponentStorage<C>>()
            .unwrap()
            .add_component(entity, component);

        entity
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
        // Run startup systems
        let startup_systems = std::mem::take(&mut self.startup_systems);

        for system in &startup_systems {
            system(self);
        }

        // Run update systems while the world is running
        let update_systems = std::mem::take(&mut self.update_systems);

        let frame_rate_sec = self.frame_rate_ms * 1e-3;
        while self.is_running {
            let now = Instant::now();

            for system in &update_systems {
                system(self);
            }

            let elapsed_time = now.elapsed().as_secs_f64();
            sleep(Duration::from_secs_f64(f64::max(
                0.0,
                frame_rate_sec - elapsed_time,
            )));
        }

        // Once the world stopped to run, run quit systems
        let quit_systems = std::mem::take(&mut self.quit_systems);

        for system in &quit_systems {
            system(self);
        }

        self.startup_systems = startup_systems;
        self.update_systems = update_systems;
        self.quit_systems = quit_systems;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }
}
