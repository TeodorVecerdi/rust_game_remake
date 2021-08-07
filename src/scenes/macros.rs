macro_rules! generate_scene_ids {
    ($idx:expr, ) => {};
    
    ($idx:expr, $head:ty) => {
        paste! {
            pub const [<$head:snake:upper>] : usize = $idx;
        }
    };
    
    ($idx:expr, $head:ty, $($tail:ty),+) => {
        paste! {
            pub const [<$head:snake:upper>] : usize = $idx;
        }
        generate_scene_ids!($idx + 1usize, $($tail),*);
    };
}

macro_rules! generate_scene_collection {
    ($($scene:ty),+) => {
        impl<'a> SceneManager<'a> {
            generate_scene_ids!(0usize, $($scene),+);
            
            pub fn new(events_loop_proxy: &'a glium::glutin::EventsLoopProxy, ui: &mut conrod_core::Ui) -> Self {
                Self {
                    scenes: vec![
                        $(
                            RefCell::new(Box::new(<$scene>::new(ui))),
                        )*
                        ],
                    current_scene: SceneManager::MAIN_MENU,
                    events_loop_proxy,
                }
            }
        }
    }
}

#[macro_export] macro_rules! generate_scene {
    ($scene:ident) => {
        pub struct $scene {
            ids: Ids,
            next_scene_index: Option<usize>,
        }
        
        impl $scene {
            pub fn new(ui: &mut Ui) -> Self {
                Self {
                    ids: Ids::new(ui.widget_id_generator()),
                    next_scene_index: None,
                }
            }
        }
    }
}