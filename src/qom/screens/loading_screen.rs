use crate::qom::screens::Screen;
use crate::qom::tiled::{parse, Map, Tileset};
use crate::qom::transitions::ScreenTransition;
use crate::qom::QuestOfMagicData;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Background::Col;
use quicksilver::load_file;
use quicksilver::prelude::{Asset, Color, Image, Window};
use quicksilver::{combinators::ok, Future};
use std::collections::HashMap;
use std::mem;
use std::path::Path;

pub const TILESET_TILES: &str = "Tiles.tsx";
pub const TILESET_CHARACTERS: &str = "Characters.tsx";
pub const MAP_OVERWORLD: &str = "Overworld.tmx";

pub struct LoadingScreenProgress {
    tileset_images_to_load: i32,
}

pub struct LoadingScreen {
    assets: Asset<(Map, Vec<(String, Asset<Image>)>)>,
    loading_progress: LoadingScreenProgress,
    images: HashMap<String, Image>,
    map: Option<Map>,
}

impl Screen for LoadingScreen {
    fn new() -> LoadingScreen {
        portable_log!("Loading screen");

        // Load Tiled map and tile sets
        let assets = Asset::new(
            load_file(Path::new(TILESET_TILES))
                .join3(
                    load_file(Path::new(TILESET_CHARACTERS)),
                    load_file(Path::new(MAP_OVERWORLD)),
                )
                .and_then(|(tileset_tiles, tileset_characters, map_overworld)| {
                    ok((
                        String::from_utf8(tileset_tiles).expect("The file must be UTF-8"),
                        String::from_utf8(tileset_characters).expect("The file must be UTF-8"),
                        String::from_utf8(map_overworld).expect("The file must be UTF-8"),
                    ))
                })
                .and_then(|(tileset_tiles, tileset_characters, map_overworld)| {
                    let mut files: HashMap<String, String> = HashMap::new();
                    files.insert(String::from(TILESET_TILES), tileset_tiles);
                    files.insert(String::from(TILESET_CHARACTERS), tileset_characters);
                    ok(parse(map_overworld.as_bytes(), &files)
                        .expect("The file must be parse-able"))
                })
                .and_then(|map| {
                    portable_log!("Got map! {:?}", map);
                    let mut image_assets: Vec<(String, Asset<Image>)> = Vec::new();
                    for tileset in &map.tilesets {
                        portable_log!("Got tileset: {:?}", tileset);
                        let tileset: &Tileset = tileset;
                        for tile in &tileset.tiles {
                            for image in &tile.images {
                                portable_log!("Found image: {:?}", image.source);
                                image_assets.push((
                                    String::from(image.source.clone()),
                                    Asset::new(Image::load(image.source.clone())),
                                ));
                            }
                        }
                    }
                    portable_log!("Images to load: {}", image_assets.len());
                    ok((map, image_assets))
                }),
        );

        LoadingScreen {
            assets,
            loading_progress: LoadingScreenProgress {
                tileset_images_to_load: 0,
            },
            images: HashMap::new(),
            map: None,
        }
    }

    fn update(&mut self, _window: &mut Window, data: &mut QuestOfMagicData) -> ScreenTransition {
        let loading_progress = &mut self.loading_progress;
        let images = &mut self.images;
        let map_optional = &mut self.map;
        // Check if loading is complete
        self.assets
            .execute(|(map, image_assets)| {
                if loading_progress.tileset_images_to_load == 0 {
                    map_optional.replace(map.to_owned());
                    // Calculate how much images are to be loaded
                    loading_progress.tileset_images_to_load = image_assets.len() as i32;
                }
                // Append images that finish
                for (source, image_asset) in image_assets {
                    image_asset.execute(|image| {
                        images.insert(source.parse().unwrap(), image.to_owned());
                        Ok(())
                    })?;
                }
                Ok(())
            })
            .unwrap();

        if loading_progress.tileset_images_to_load > 0
            && images.len() == loading_progress.tileset_images_to_load as usize
        {
            // Put map into the game data
            mem::swap(&mut self.map, &mut data.overworld_map);
            // Put the images into the game data
            mem::swap(&mut self.images, &mut data.image_assets);
            // return transition to flip to next screen
            //todo
        }
        ScreenTransition::None
    }

    fn draw(&mut self, window: &mut Window) {
        // Draw loading progress
        let width = window.screen_size().x;
        let height = window.screen_size().y;
        let full_width = width - 200.0;
        // Full progress bar
        window.draw(
            &Rectangle::new((100, height - 100.0), (full_width, 50)),
            Col(Color::from_rgba(50, 50, 50, 1.0)),
        );
        // Partial progress bar
        let progress = if self.loading_progress.tileset_images_to_load == 0 {
            0.0
        } else {
            self.images.len() as f32 / self.loading_progress.tileset_images_to_load as f32
        };
        window.draw(
            &Rectangle::new((100, height - 100.0), (full_width * progress, 50)),
            Col(Color::from_rgba(75, 200, 75, 1.0)),
        );
    }
}
