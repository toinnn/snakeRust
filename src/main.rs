#![allow(unused)]
use core::fmt;
use std::{ vec};
use Coisa::{Cor , Number}  ;
use rand::Rng;
use Direction::{left,right ,up ,down};
use bevy::prelude::*;
// use bevy::sprite::entity::*;
use noise::Perlin;
use noise::NoiseFn;
use perlin_noise::PerlinNoise;
// use utilities::map;




pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

#[derive(Debug)]
enum Coisa{
    Cor(String),
    Number(i32)
}
impl fmt::Display for Coisa{
    fn fmt(&self ,f :&mut fmt::Formatter) -> fmt::Result{ 
        // format!();
        write!(f,"{:?}",self)
        
    }
}
enum Direction{
    left ,
    right,
    up,
    down,
}
struct position{
    x  : u16,
    y  : u16
}
impl PartialEq for position{
    fn eq(&self , other : &Self) -> bool {
        if self.x == other.x && self.y == other.y{
            return true;
        }else {
            return false;
        }

    }
}
struct snake{
    body : Vec<position>,
    lastDirection : Direction
}
impl snake {
    pub fn Next_dir(self,dir: i8) -> Direction {
    
            match dir  {
            1 => {
                snake{lastDirection:Direction::left , ..self};
                return self.lastDirection ;
            },
            2 => {
                snake{lastDirection : Direction::right , ..self};
                return self.lastDirection ;
            },
            3 => {
                snake{lastDirection : Direction::up , ..self};
                return self.lastDirection ;
            },
            4 => {
                snake{lastDirection : Direction::down , ..self};
                return self.lastDirection ;
            },
            _ => {
                format!("Tecla não prevista digitada foi : {}",dir);
                return self.lastDirection ;
            },
        }
    }
}
struct field{
    size_X:u16,
    size_y:u16,
    food_Pos:position
}
impl field {
    fn new_Food(self , cobra_body :Vec<position>){
        let mut rng = rand::thread_rng();
        let mut new_food = position{x : rng.gen_range(1..self.size_X) , y : rng.gen_range(1..self.size_y) } ;
        loop {
            if  !cobra_body.contains(&new_food)   {
                break;
            }
            let mut new_food = position{x : rng.gen_range(1..self.size_X) , y : rng.gen_range(1..self.size_y) } ;
        }
        field{food_Pos : new_food , ..self};
        
    }
}
struct game_session{
    points : u16,
    record : u16,
    player : snake,
    ground : field,
}
impl game_session {
    pub fn new(x_field_size :u16 ,y_field_size :u16 )-> Self {
        let mut rng = rand::thread_rng();
        let cobra = snake{body : vec![position{ x:rng.gen_range(1..x_field_size-1) ,
        y : rng.gen_range(1..y_field_size)}] , lastDirection : right };
        
        let mut new_food = position{x : rng.gen_range(1..x_field_size) , y : rng.gen_range(1..y_field_size) } ;
        loop {
            if !cobra.body.contains(&new_food) {
                break;
            }
            let mut new_food = position{x : rng.gen_range(1..x_field_size) , y : rng.gen_range(1..y_field_size) } ;
        }
        
        let land = field { size_X : x_field_size  , size_y : y_field_size, food_Pos : new_food };
        return game_session{points : 0 , record :0 , player :cobra ,ground : land};

    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

// pub struct genericaPlugin;
// impl Plugin for genericaPlugin{
//     fn build(&self, app: &mut App,
//         mut commands: Commands,
//         asset_server: Res<AssetServer>,
//         mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
//         // app.insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
//         //     .insert_resource(WindowDescriptor{
//         //         title:"TELA SUPIMPA !!".to_string(),
//         //         width: 400.0,
//         //         height: 200.0,
//         //         vsync: true, 
//         //         // position : Some(Vec2::new(-500.0,200.0)),
//         //         ..Default::default()
//         //     });
//         let texture_handle = asset_server.load("Fi_Do_Bowser//FiDoBowser_SpriteSheet.bmp");
//         let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 57.0), 24, 1);
//         let texture_atlas_handle = texture_atlases.add(texture_atlas);
//         commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//         commands
//             .spawn_bundle(SpriteSheetBundle {
//                 texture_atlas: texture_atlas_handle,
//                 transform: Transform::from_scale(Vec3::splat(6.0)),
//                 ..Default::default()
//             })
//             .insert(Timer::from_seconds(0.1, true));
//     }

// }
fn spawnAnimado(path : &str ,
    spriteScale : f32,
    worldScale : f32 ,
    translation_X : f32,
    translation_Y : f32,
    translation_Z : f32,

    sprite_Tile_Size : Vec2,   
    sprite_Sheet_Columns : u8,
    sprite_Sheet_rows : u8 ,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    )
{
    let Fi_scale = spriteScale;
        let texture_handle = asset_server.load(path);
        let texture_atlas = TextureAtlas::from_grid(texture_handle ,
            sprite_Tile_Size , sprite_Sheet_Columns.into() , sprite_Sheet_rows.into());
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(Fi_scale)) 
                .with_translation(Vec3::new(worldScale*translation_X , translation_Y , translation_Z )),
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.1, true));
}    
fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows : ResMut<Windows>,mut materials : ResMut<Assets<StandardMaterial>>
) {
    
    let texture_handle = asset_server.load("grass_tileset_16x16//grass_tileset_16x16.png");
    
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 9, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    // let texture_handle = asset_server.load("Fi_Do_Bowser//FdB-1.png").unwrap();
    // let material_handle = materials.add();
    // let texture_atlas = TextureAtlas::from_grid(texture_handle  , Vec2::new(72.0, 57.0), 1, 1);
    // let texture_atlas_handle = texture_atlas.add_texture(rect) .add(texture_atlas);

    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(0,0));
    window.set_title( "TELA SUPIMPA !!".to_string() );
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    let scale = 2.0;
    let mut ctd = 0; 
    for i in 0..9 {
        for j  in 1..10 {
            commands.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(scale*17.0*j as f32 , scale*-17.0*i as f32 , 0.0))
                    .with_scale(Vec3::splat(scale)),
                sprite: TextureAtlasSprite::new(8),
                ..Default::default()} ); 
            commands.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(scale*17.0*j as f32 , scale*-17.0*i as f32 , 0.0))
                    .with_scale(Vec3::splat(scale)),
                sprite: TextureAtlasSprite::new(ctd),
                ..Default::default()} );        
            ctd = ctd + 1;
        }

            }
    // commands.spawn_bundle(SpriteBundle {
    //     // texture: asset_server.load("d9cuskc-2ec59299-0a41-4c29-813c-cdd3dfb44b6e.gif").into() ,
    //     texture: asset_server.load("Fi_Do_Bowser//FdB-1.png").into() ,
    //     // texture: textura.base_color_texture.unwrap() ,
    //     transform: Transform::from_translation(Vec3::new(scale*30.0 , -17.0 , 1.0)),
    //     ..Default::default() });
    let Fi_scale = 1.0/1.2;
    let texture_handle = asset_server.load("Fi_Do_Bowser//FiDoBowser_SpriteSheet.bmp");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 57.0), 24, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(Fi_scale)).with_translation(Vec3::new(scale*27.5 , -17.0 , 1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
    
    let Fi_scale = 1.0/1.2;
    let texture_handle = asset_server.load("Gif_Sprite_Sheets//Mario&Yoshi_Gif_SpriteSheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(62.0, 92.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    commands
        .spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(Fi_scale)).with_translation(Vec3::new(scale*47.5 , -17.0 , 1.0)),
        ..Default::default()})
        .insert(Timer::from_seconds(0.1, true));
        
    // spawnAnimado("Fi_Do_Bowser//FiDoBowser_SpriteSheet.bmp",1.0/1.2 ,2.0,
    //     27.5 , -17.0 , 1.0,
    //     Vec2::new(72.0, 57.0), 24, 1,
    //     commands , asset_server , texture_atlases);

    // spawnAnimado("Fi_Do_Bowser//Yoshi_SpriteSheet.png" , 1.0/1.2 ,2.0 ,
    //     27.5 , 15.0*-17.0 , 1.0 ,
    //     Vec2::new(52.0, 68.0), 12, 1,
    //     commands , asset_server , texture_atlases);
    let perli = Perlin::new();
    let val = map(perli.get([ 37.7, 2.8]) as f32  , -1.0 ,1.0 , 0.0 , 1.0) ;

    let perlin = PerlinNoise::new();
    println!("{}",val );
    println!("{}",perlin.get2d([12.0,32.0]));
    //async
    for i in 0..300 {
        for j  in 0..200 {
        let valor = 20.0*map(perlin.get2d([ 0.01*i as f64 , 0.01*j as f64 ]) as f32  , -1.0 ,1.0 , 0.0 , 1.0) ;
        // println!("{}",valor ); 
        // let valor = perlin.get2d([i as f64 , j as f64 ]); 
        // valor = valor ;
    commands.spawn_bundle(SpriteBundle{ 
        sprite : Sprite{color : Color::rgba(0.04 , 0.04 , 0.04 , valor as f32 ) ,custom_size : Vec2::new(2.0,2.0 ).into() ,
             .. Default::default()},transform: Transform::from_translation(Vec3::new(-650.0 + 2.0*i as f32 ,
                320.0 - 2.0*j as f32 , 2.0)) ,
         ..Default::default()
    });
    }} 
    
    // commands.spawn().insert_bundle(SpriteComponents {
    //     material : materials.add(texture_handle.into()),
    //     transform: Transform::from_translation(Vec3::new(17.0 , -17.0 , 0.0)),
    //     ..Default::default()} );
    
    
}


fn main() {

    let _num64 = 32f64;
    let mut _cores = vec![Cor("greem".to_string()) , Number(32)];     //let mut _cores = vec![Coisa::Cor(13),Coisa::Number(12)];
    println!("o número é {}\na cor é {}",_cores[1],_cores[0]);
    // App::new()
    //     .add_plugin(CorePlugin::default())
    //     .add_plugin(InputPlugin::default())
    //     .add_plugin(WindowPlugin::default())
    //     .run();
    
    
    
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(genericaPlugin)      
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .add_startup_system(setup.system()) 
        .add_system(animate_sprite_system)
        .run();
}