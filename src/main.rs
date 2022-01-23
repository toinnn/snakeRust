use core::fmt;
use std::{ vec};
use Coisa::{Cor , Number}  ;
use rand::Rng;
use Direction::{left,right ,up ,down};
use bevy::prelude::*;

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

pub struct genericaPlugin;
impl Plugin for genericaPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
            .insert_resource(WindowDescriptor{
                title:"TELA SUPIMPA !!".to_string(),
                ..Default::default()
            });
    }

}
fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("grass_tileset_16x16//grass_tileset_16x16.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 9, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    let mut ctd = 0;
    for i in (0..9) {
        for j  in (1..10) {
            commands.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(17.0*j as f32 , -17.0*i as f32 , 0.0)),
                sprite: TextureAtlasSprite::new(8),
                ..Default::default()} ); 
            commands.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(17.0*j as f32 , -17.0*i as f32 , 0.0)),
                sprite: TextureAtlasSprite::new(ctd),
                ..Default::default()} );        
            ctd = ctd + 1;
        }
    }
    /*
    ///ROW : 0
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -22.0, 0.0)),
        sprite: TextureAtlasSprite::new(0),
        ..Default::default()} );
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(21.0, -22.0, 0.0)),
        sprite: TextureAtlasSprite::new(1),
        ..Default::default()} );
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(42.0, -22.0, 0.0)),
        sprite: TextureAtlasSprite::new(2),
        ..Default::default()} );
    ///ROW : 1
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -43.0, 0.0)),
        sprite: TextureAtlasSprite::new(3),
        ..Default::default()} );
    
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(21.0, -43.0, 0.0)),
        sprite: TextureAtlasSprite::new(4),
        ..Default::default()} );
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(42.0, -43.0, 0.0)),
        sprite: TextureAtlasSprite::new(5),
        ..Default::default()} );
    ///ROW : 2
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -64.0, 0.0)),
        sprite: TextureAtlasSprite::new(6),
        ..Default::default()} );
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(21.0, -64.0, 0.0)),
        sprite: TextureAtlasSprite::new(7),
        ..Default::default()} );
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(42.0, -64.0, 0.0)),
        sprite: TextureAtlasSprite::new(8),
        ..Default::default()} );
    */
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
        .add_plugin(genericaPlugin)      
        .add_startup_system(setup.system()) 
        .run();
}