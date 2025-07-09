//! 一种将资源句柄集合作为资源加载的高级方法。

use std::collections::VecDeque;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(PreUpdate, load_resource_assets);
}

pub trait LoadResource {
    /// 这将把 [`Resource`] 作为 [`Asset`] 加载。当其所有的资源依赖项
    /// 都已加载时，它将作为资源插入。这确保了资源仅在
    /// 资源准备就绪时存在。
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

/// 一个函数，用于插入已加载的资源。
type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    // 使用队列来存储等待的资源，以便可以逐一循环处理并移动到
    // `finished`。
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    /// 如果所有请求的 [`Asset`] 都已完成加载并可作为 [`Resource`] 使用，则返回 true。
    pub fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}
