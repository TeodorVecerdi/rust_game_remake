macro_rules! generate_scene_ids {
    ($idx:expr, ) => {};
    ($idx:expr, $head: ty, $($tail:ty)*) => {
        paste! {
            pub const [<$head:snake:upper>] : usize = $idx;
        }
        generate_scene_ids!($idx + 1usize, $($tail,)*);
    };
}

macro_rules! generate_scene_collection {
    ($($scene:ty),+) => {
        impl SceneManager {
            generate_scene_ids!(0usize, $($scene),*);
            
            pub fn new(ui: &mut conrod_core::Ui) -> Self {
                Self {
                    scenes: vec![
                        $(
                            RefCell::new(Box::new(<$scene>::new(ui))),
                        )*
                        ],
                    current_scene: SceneManager::MAIN_MENU,
                }
            }
        }
    }
}