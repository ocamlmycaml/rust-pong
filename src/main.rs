use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::input::{self, Key};
use tetra::{
    Context,
    ContextBuilder,
    State
};

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity { texture, position, velocity }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height()
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}


struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let window_height = tetra::window::get_height(ctx);
        let window_width = tetra::window::get_width(ctx);

        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (window_height - player1_texture.height()) as f32 / 2.0
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            (window_width - 16 - player2_texture.width()) as f32,
            (window_height - player2_texture.height()) as f32 / 2.0
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            window_width as f32 / 2.0 - ball_texture.width() as f32 / 2.0,
            window_height as f32 / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let window_height = tetra::window::get_height(ctx) as f32;

        let player1_max_pos = window_height - self.player1.texture.height() as f32;
        let player2_max_pos = window_height - self.player2.texture.height() as f32;

        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y = (0.0 as f32).max(self.player1.position.y - PADDLE_SPEED);
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y = player1_max_pos.min(self.player1.position.y + PADDLE_SPEED);
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y = (0.0 as f32).max(self.player2.position.y - PADDLE_SPEED);
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y = player2_max_pos.min(self.player2.position.y + PADDLE_SPEED);
        }

        self.ball.position += self.ball.velocity;

        // collision detection lol

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= window_height {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x < 0.0 {
            tetra::window::quit(ctx);
            println!("Player 2 wins!");
        }

        if self.ball.position.x > tetra::window::get_width(ctx) as f32 {
            tetra::window::quit(ctx);
            println!("Player 1 wins!");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 400)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
