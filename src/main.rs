#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use std::collections::VecDeque;
const NUMBER_OF_SQUARES: i32 = 10;


fn window_conf() -> Conf{
    Conf { window_title: "Snake".to_owned(), 
    window_width:  600,
    window_height: 600,
    window_resizable: false,
    icon: None,
    ..Default::default()}
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    
    loop {
        game.play();
        next_frame().await
    }
    
}

enum State{
    Menu,
    Playing,
    DeathScreen
}

struct Game{
    gamestate: State,
    snake: Snake,
    food: Food,
    death_time: Option<f64>
}

fn rem(x: f64, y:f64) -> f64{
    x - (x/y).floor()
}

impl Game{
    fn new() -> Game{
        let gamestate = State::Menu;
        let snake = Snake::new();
        let food = Food::rand(snake.positions());
        Game{gamestate, snake, food, death_time: None}
    }

    fn play(&mut self) {
        match self.gamestate{
            State::Menu        => self.menu(),
            State::Playing     => self.playing(),
            State::DeathScreen => self.death_screen()
        }
    }

    fn menu(&mut self) {
        let font_size = (screen_height()/2.5) as u16;
        let text_size = measure_text("SNAKE", None, font_size, 1.0);
        draw_text("SNAKE", screen_width()/2. - text_size.width/2., screen_height()/3.5, font_size as f32, GREEN);
        
        let text= if rem(get_time(), 1.0) > 0.5{
             "Press enter to start "
        }
        else {
            "Press enter to start."
        };
        let text_size = measure_text(text, None, 40, 1.0);
        draw_text(text, screen_width()/2.0 - text_size.width/2.0, screen_height()*0.75, 40.0, DARKGRAY);

        if is_key_down(KeyCode::Enter){
            self.gamestate = State::Playing;
        }


    }
    
    
    fn playing(&mut self){
        
        if is_key_pressed(KeyCode::Up) && ! matches!(self.snake.facing, Direction::Up) && (! matches!(self.snake.last_facing, Direction::Down) || self.snake.body.len() == 0) && self.snake.alive{
            self.snake.facing = Direction::Up;
            self.snake.last_movement = self.snake.movement_delay + 1.0;
        }
        if is_key_pressed(KeyCode::Down) && ! matches!(self.snake.facing, Direction::Down) && (! matches!(self.snake.last_facing, Direction::Up) || self.snake.body.len() == 0) && self.snake.alive{
            self.snake.facing = Direction::Down;
            self.snake.last_movement = self.snake.movement_delay + 1.0;
        }
        if is_key_pressed(KeyCode::Right) && ! matches!(self.snake.facing, Direction::Right) && (! matches!(self.snake.last_facing, Direction::Left) || self.snake.body.len() == 0) && self.snake.alive{
            self.snake.facing = Direction::Right;
            self.snake.last_movement = self.snake.movement_delay + 1.0;
        }
        if is_key_pressed(KeyCode::Left) && ! matches!(self.snake.facing, Direction::Left) && (! matches!(self.snake.last_facing, Direction::Right) || self.snake.body.len() == 0) && self.snake.alive{
            self.snake.facing = Direction::Left;
            self.snake.last_movement = self.snake.movement_delay + 1.0;
        }
        if self.snake.head == self.food.pos{
            self.snake.grow();
            self.food = Food::rand(self.snake.positions());
        }

            self.snake.update();

            if ! self.snake.alive {
                match self.death_time{
                    None => self.death_time = Some(get_time()),
                    Some(time) => if get_time()-time > 3.0 {self.gamestate = State::DeathScreen;}
                }
            }
            
            
            self.main_drawing();
        }

          
        
    fn main_drawing(&mut self) {
        clear_background(BLACK);
        let rectangle_size = (screen_width()/(NUMBER_OF_SQUARES as f32), screen_height()/(NUMBER_OF_SQUARES as f32));

            for i in 1..NUMBER_OF_SQUARES{
                let x = rectangle_size.0*(i as f32);
                draw_line(x, 0., x, screen_height(), 1., GRAY);
            }

            for j in 1..NUMBER_OF_SQUARES{
                let y = rectangle_size.1*(j as f32);
                draw_line(0., y, screen_width(), y, 1., GRAY);
            }

            self.snake.draw();
            if self.snake.alive{
                self.food.draw();
            }
    }
    fn death_screen(&mut self) {
            self.main_drawing();
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.5, 0.5, 0.5, 0.3));
            let font_size = screen_height()/3.7;
            let text_size = measure_text("YOU DIED", None, font_size as u16, 1.0);
            draw_text("YOU DIED", screen_width()/2.0-text_size.width/2.0 , screen_height()/2.0+text_size.height/2.0, font_size, Color::new(0.8, 0.0, 0.0, 1.0));

            let text = if rem(get_time(), 1.0)>0.5 {"Press enter to retry."} else {"Press enter to retry "};

            let enter_size = measure_text(text, None, 40, 1.0);
            draw_text(text, screen_width()/2.0 - enter_size.width/2.0, screen_height()/2.0+text_size.height/2.0+enter_size.height+10.0, 40.0, GREEN);

            let text = &format!("You ate {} cookies", self.snake.body.len());
            let font_size = 50.0;
            let text_size = measure_text(text, None, font_size as u16, 1.0);
            draw_text(text, screen_width()/2.0 - text_size.width/2.0, screen_height() - 20.0, font_size, Color::new(0.4117, 0.0, 0.0, 1.0));

            if is_key_down(KeyCode::Enter) {
                self.snake = Snake::new();
                self.food = Food::rand(self.snake.positions());
                self.death_time = None;
                self.gamestate = State::Playing;
            }
        }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Pos{ x: i32, y:i32 }

impl Pos{
    fn new(x: i32, y: i32) -> Pos{
        Pos{x, y}
    }

    fn moove(&self, other: &Pos) -> Pos{
        Pos{x: self.x + other.x, y: self.y+other.y}
    }

    fn mooveip(&mut self, other: &Pos) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct Snake{
    head: Pos,
    facing: Direction,
    alive: bool,
    body: VecDeque<Pos>,
    last_facing: Direction,
    last_movement: f64,
    movement_delay: f64
}


impl Snake{
    fn new() -> Snake{
        Snake{
            head: Pos::new(0, 0),
            facing: Direction::Right,
            alive: true,
            body: VecDeque::new(),
            last_facing: Direction::Right,
            last_movement: get_time(),
            movement_delay: 1.0
        }
    }

    fn grow(&mut self) {
        self.body.push_back(Pos::new(-10, -10));
        self.movement_delay = match self.body.len(){
            0       => 1.,
            1..=3   => 0.8,
            4..=5   => 0.6,
            6..=7   => 0.4,
            8..=10  => 0.3,
            11..=20 => 0.2,
            _       => 0.1
        };
    }

    fn positions(&self) -> VecDeque<Pos>{
        let mut vec = self.body.clone();
        vec.push_front(self.head.clone());
        vec
    }

    fn update(&mut self) {
        if get_time() - self.last_movement > self.movement_delay && self.alive{
            self.last_movement = get_time();
            self.last_facing = self.facing.clone();
            let mut next_pos = self.head.clone();
            match self.facing {
                Direction::Up    => next_pos.y -= 1,
                Direction::Down  => next_pos.y += 1,
                Direction::Left  => next_pos.x -= 1,
                Direction::Right => next_pos.x += 1
            };

            if next_pos.x >= NUMBER_OF_SQUARES as i32{
                next_pos.x = 0;
            }
            else if next_pos.x < 0 {
                next_pos.x = NUMBER_OF_SQUARES as i32 -1;
            }
            
            if next_pos.y >= NUMBER_OF_SQUARES as i32{
                next_pos.y = 0;
            }
            else if next_pos.y < 0 {
                next_pos.y = NUMBER_OF_SQUARES as i32 -1;
            }
            
            if self.body.len() > 3{
                if self.body.contains(&next_pos) {
                    self.alive = false;
                }
            }
            if self.alive{
                // Mover el cuerpo
                if self.body.len() > 0{
                    self.body.push_front(self.head.clone());
                    self.body.pop_back();
                }
                self.head = next_pos;
            }
        }
    }

    fn draw(&self) {
        let rectangle_size = (screen_width()/(NUMBER_OF_SQUARES as f32), screen_height()/(NUMBER_OF_SQUARES as f32));
        let (color, tail_color) = if self.alive {(GREEN, DARKGREEN)} else {(RED, Color::new(0.5, 0., 0., 1.))};
         
        for element in &self.body{
            draw_rectangle(element.x as f32 * rectangle_size.0, element.y as f32 * rectangle_size.1, rectangle_size.0, rectangle_size.1, tail_color);
        }
        draw_rectangle(self.head.x as f32 * rectangle_size.0, self.head.y as f32 * rectangle_size.1, rectangle_size.0, rectangle_size.1, color);

        let mut center = self.head.clone();
        center.x *= rectangle_size.0 as i32;
        center.y *= rectangle_size.1 as i32;

        
        center.mooveip(&Pos::new((rectangle_size.0/2. - rectangle_size.0/10.) as i32,
                                                    (rectangle_size.1/2. - rectangle_size.1/10.) as i32));

        let left_eye = match self.facing{
            Direction::Up    => center.moove(&Pos::new((-1./4.*rectangle_size.0) as i32, (-1./4.*rectangle_size.1) as i32)),

            Direction::Down  => center.moove(&Pos::new((1./4.*rectangle_size.0) as i32, (1./4.*rectangle_size.1) as i32)),

            Direction::Left  => center.moove(&Pos::new((-1./4.*rectangle_size.0) as i32, (1./4.*rectangle_size.1) as i32)),
            
            Direction::Right => center.moove(&Pos::new((1./4.*rectangle_size.0) as i32, (-1./4.*rectangle_size.1) as i32))
        };


        let right_eye = match self.facing{
            Direction::Up    => center.moove(&Pos::new((1./4.*rectangle_size.0) as i32, (-1./4.*rectangle_size.1) as i32)),

            Direction::Down  => center.moove(&Pos::new((-1./4.*rectangle_size.0) as i32, (1./4.*rectangle_size.1) as i32)),

            Direction::Left  => center.moove(&Pos::new((-1./4.*rectangle_size.0) as i32, (-1./4.*rectangle_size.1) as i32)),
            
            Direction::Right => center.moove(&Pos::new((1./4.*rectangle_size.0) as i32, (1./4.*rectangle_size.1) as i32))
        };
        
        draw_rectangle(right_eye.x as f32, right_eye.y as f32, rectangle_size.0/10., rectangle_size.1/10.0, BLACK);
        draw_rectangle(left_eye.x as f32, left_eye.y as f32, rectangle_size.0/10., rectangle_size.1/10.0, BLACK);

    }
}

#[derive(Clone)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Food{
    pos: Pos
}

impl Food{
    fn rand(not_permited: VecDeque<Pos>) -> Food{
        let mut pos;
        loop{
            pos = Pos::new(rand::gen_range(0, NUMBER_OF_SQUARES), rand::gen_range(0, NUMBER_OF_SQUARES));
            if ! not_permited.contains(&pos){
                break
            }
        }
        Food{pos}
    }

    fn draw(&self) {
        let rectangle_size = (screen_width()/(NUMBER_OF_SQUARES as f32), screen_height()/(NUMBER_OF_SQUARES as f32));
        draw_rectangle(self.pos.x as f32 * rectangle_size.0, self.pos.y as f32 * rectangle_size.1, rectangle_size.0, rectangle_size.1, BROWN);
    }
}