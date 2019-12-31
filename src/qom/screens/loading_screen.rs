use crate::qom::screens::Screen;
use crate::qom::tiled::{parse, parse_file, parse_tileset};
use crate::qom::QuestOfMagicData;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Background::Col;
use quicksilver::prelude::{Asset, Color, Image, Window};
use quicksilver::{combinators::ok, Future};
use quicksilver::{load_file, Result};
use std::collections::HashMap;
use std::path::Path;

pub const TILESET_TILES: &str = "Tiles.tsx";
pub const TILESET_CHARACTERS: &str = "Characters.tsx";
pub const MAP_OVERWORLD: &str = "Overworld.tsx";

pub struct LoadingScreen {
    test_asset: Asset<Vec<u8>>,
}

impl Screen for LoadingScreen {
    fn new() -> LoadingScreen {
        portable_log!("Loading screen");
        // Load assets

        /*
        //let file = File::open(&Path::new("Overworld.tmx")).unwrap();
        //portable_log!("Opened file");
        //let reader = BufReader::new(file
        //portable_log!("reader {:?}", reader);
        let map = parse_file(&Path::new("Overworld.tmx")).unwrap();
        portable_log!("map {:?}", map);
        portable_log!();
        portable_log!("layers {:?}", map.layers);
        portable_log!();
        //map.layers
        //map.get_tileset_by_gid( )
        let tileset = map.get_tileset_by_gid(1).unwrap();
        portable_log!("tileset by gid{:?}", tileset);
        */
        //let asset = Asset::new(Image::load("image.png"));

        //let map = parse_file(&Path::new("Overworld.tmx")).unwrap();
        //let path = Path::new("Overworld.tmx");
        /*
                let mut asset_file = Asset::new(load_file(path));
                asset_file.execute(|a| {
                    portable_log!("file: {:?}", a);
                    Ok(())
                });
        */
        let test_asset = Asset::new(
            load_file(Path::new(TILESET_TILES)), //.and_then(|a| load_file(Path::new(TILESET_CHARACTERS)).and_then(|b| ok((a, b)))),
        );
        /*
        load_file(Path::new(TILESET_TILES)).join3(
            load_file(Path::new(TILESET_CHARACTERS)),
            load_file(Path::new(MAP_OVERWORLD)),
        )*//*
           .and_then(|(tileset_tiles, tileset_characters, map_overworld)| {
               ok((tileset_tiles, tileset_characters, map_overworld))
           }),*/
        /*
        .and_then(|tileset_tiles| {
            load_file(Path::new(TILESET_CHARACTERS)).and_then(|tileset_characters| {
                load_file(Path::new(MAP_OVERWORLD)).and_then(|map_overworld| {
                    ok((tileset_tiles, tileset_characters, map_overworld))
                })
            })
        })
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
            ok(true)
        }), //.and_then(|image_path| Image::load(image_path))
        */

        LoadingScreen {
            test_asset: test_asset,
        }
    }

    fn update(&mut self, _window: &mut Window, data: &mut QuestOfMagicData) -> Result<()> {
        // Check if loading is complete

        self.test_asset.execute_or(
            |b| {
                portable_log!("Loaded!");
                Ok(())
            },
            || {
                portable_log!("Not loaded yet");
                Ok(())
            },
        );

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
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
        let progress = 0.25;
        window.draw(
            &Rectangle::new((100, height - 100.0), (full_width * progress, 50)),
            Col(Color::from_rgba(75, 200, 75, 1.0)),
        );

        /*
        window.draw(&Rectangle::new((100 + self.x as i32, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400 + self.x as i32, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        window.draw(&Circle::new((400 + self.x as i32, 300), 100), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50 + self.x as i32, 80),(600 + self.x as i32, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500 + self.x as i32, 50), (450 + self.x as i32, 100), (650 + self.x as i32, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        */

        Ok(())
    }
}
