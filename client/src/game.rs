use engine::World;
use glam::Mat4;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::model::Model;
use crate::network::{Connection, MessageType};

pub type Pos = (f64, f64);
const LEVEL_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/ramp.obj");
const PLAYER_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/Ball.obj");
pub struct Game {
    background_color: wgpu::Color,
    own_pointer_pos: Pos,
    remote_pointer_pos: Pos,
    has_remote: bool,
    connection: Connection,
    world: World,
    model: Option<Model>,
    player: Option<Model>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            background_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            has_remote: false,
            own_pointer_pos: Default::default(),
            remote_pointer_pos: Default::default(),
            connection: Connection::Pairing(fetch::Connection::new().ok()),
            world: World::new(LEVEL_PATH),
            model: None,
            player: None,
        }
    }
}

impl Game {
    pub fn tick(&mut self) {
        self.exchange_positions();
        self.world.simulate_step();
        let (x_scale, y_scale) = match self.connection.update(self.own_pointer_pos) {
            None if !self.has_remote => self.own_pointer_pos,
            None => self.remote_pointer_pos,
            Some(new) => {
                self.remote_pointer_pos = new;
                self.has_remote = true;
                new
            }
        };
        self.background_color = wgpu::Color {
            r: x_scale,
            g: (1.0 - x_scale),
            b: y_scale,
            a: (1.0 - y_scale),
        };
    }

    pub fn load_model(
        &mut self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> anyhow::Result<()> {
        self.model = Some(crate::model::load_model(LEVEL_PATH, device, queue, layout)?);
        Ok(())
    }

    pub fn load_player(
        &mut self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> anyhow::Result<()> {
        self.player = Some(crate::model::load_model(
            PLAYER_PATH,
            device,
            queue,
            layout,
        )?);
        Ok(())
    }

    pub fn update_pointer_pos(&mut self, pos: Pos) {
        self.connection.send(MessageType::MousePosition(pos));
        self.own_pointer_pos = pos;
    }
    pub fn background_color(&self) -> wgpu::Color {
        self.background_color
    }

    pub fn model(&self) -> Option<&Model> {
        self.model.as_ref()
    }
    pub fn player(&self) -> Option<&Model> {
        self.player.as_ref()
    }

    pub fn player_tranform(&self) -> Mat4 {
        let trans = self.world.player_position();
        let rot = self.world.player_rotation();
        glam::Mat4::from_translation(trans) * glam::Mat4::from_quat(rot)
    }

    pub fn exchange_positions(&mut self) {
        if !self.connection.is_connected() {
            self.connection.pair();
        } else {
            for update in self.connection.fetch_updates() {
                #[expect(irrefutable_let_patterns)]
                if let MessageType::MousePosition(pos) = update.content {
                    self.remote_pointer_pos = pos;
                }
            }
        }
    }
}
