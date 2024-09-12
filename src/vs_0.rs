use crate::audio;
use crate::instance::Instance;
use crate::model::{InstanceModel, Model};
use crate::pipeline::Pipeline;
use crate::pipeline::{PipelineError, PipelineGroup};
use cgmath::Rotation3;
use cgmath::Zero;
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Show {
    Lua,
    MariusJulien,
}

use Show::*;

const COLOR_0_0: [u8; 4] = [0x9f, 0x56, 0xff, 0xff];
const COLOR_1_0: [u8; 4] = [0xb5, 0x82, 0xff, 0xff];
const COLOR_2_0: [u8; 4] = [0xca, 0xad, 0xff, 0xff];
const COLOR_3_0: [u8; 4] = [0xff, 0xad, 0xc7, 0xff];
const COLOR_4_0: [u8; 4] = [0xff, 0x99, 0xb6, 0xff];

const COLOR_0_1: [u8; 4] = [0x9f, 0x86, 0xfa, 0xff];
const COLOR_1_1: [u8; 4] = [0x60, 0x64, 0xfc, 0xff];
const COLOR_2_1: [u8; 4] = [0x1b, 0x59, 0xff, 0xff];
const COLOR_3_1: [u8; 4] = [0x00, 0x05, 0xf1, 0xff];
const COLOR_4_1: [u8; 4] = [0x2f, 0x08, 0x85, 0xff];

const COLORS_0: [[u8; 4]; 5] = [COLOR_0_0, COLOR_1_0, COLOR_2_0, COLOR_3_0, COLOR_4_0];
const COLORS_1: [[u8; 4]; 5] = [COLOR_0_1, COLOR_1_1, COLOR_2_1, COLOR_3_1, COLOR_4_1];

const COLOR_SHADING_PERIOD: f64 = 3600.0;

fn get_color_0(rng: &mut ThreadRng) -> [f32; 4] {
    let v = COLORS_0
        .choose(rng)
        .unwrap()
        .iter()
        .map(|c| hex_to_f(*c))
        .collect::<Vec<_>>();
    [v[0], v[1], v[2], v[3]]
}
fn get_color_1(rng: &mut ThreadRng) -> [f32; 4] {
    let v = COLORS_1
        .choose(rng)
        .unwrap()
        .iter()
        .map(|c| hex_to_f(*c))
        .collect::<Vec<_>>();
    [v[0], v[1], v[2], v[3]]
}

fn get_switch_time(time: f32, rng: &mut ThreadRng) -> f32 {
    600.0 * rng.gen::<f32>() + 600.0 + time
}

fn deactivate_pipeline(pipeline: &mut Pipeline) {
    for i_m in &mut pipeline.instance_models {
        for i in &mut i_m.instances {
            i.scale = 0.0;
            i.position[0] = 0.0;
        }
    }
}

pub const POST_SHADER_0: &str = include_str!("../shader/vs_0/post_0.wgsl");
pub const POST_SHADER_1: &str = include_str!("../shader/vs_0/post_1.wgsl");
const NB_DISKS: usize = 4;
const DISK_SPEED: f32 = 0.3;

pub struct State {
    noise_3d_activated: bool,
    noise_3d_start_time: f32,
    noise_3d_duration: f32,

    full_activated: (bool, usize),
    full_start_time: f32,
    full_duration: f32,

    wf_3d_activated: (bool, usize),
    wf_3d_start_time: f32,
    wf_3d_duration: f32,
    wf_3d_axis: cgmath::Vector3<f32>,

    disk_activated: [bool; NB_DISKS],
    disk_start_time: [f32; NB_DISKS],
    disk_duration: [f32; NB_DISKS],
    disk_scale: [f32; NB_DISKS],

    dyn_pipelines: Vec<usize>,
    active_pipelines: [usize; audio::NB_AUDIO_CHANNELS],
    pipeline_switch_time: f32,
    show: Show,
    rng: ThreadRng,
}

impl State {
    pub fn new(
        pipeline_group: &mut PipelineGroup,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        show: Show,
    ) -> Result<State, PipelineError> {
        let quad = Model::new_quad(device);
        let instance = Instance::new();
        let instance_model = InstanceModel::new(quad, vec![instance], device);

        let shader = match show {
            Lua => include_str!("../shader/vs_0/wallpaper_noise_0.wgsl"),
            MariusJulien => include_str!("../shader/vs_0/wallpaper_noise_1.wgsl"),
        };

        pipeline_group.add_pipeline(vec![instance_model], shader, device, config)?;

        let quad: Model = Model::new_quad(device);
        let mut instance = Instance::new();
        instance.scale = 0.2;
        let instance_model = InstanceModel::new(quad, vec![instance], device);

        pipeline_group.add_pipeline(
            vec![instance_model],
            include_str!("../shader/vs_0/2d_logo.wgsl"),
            device,
            config,
        )?;

        let quad = Model::new_quad(device);
        let instance = Instance::new();
        let instance_model = InstanceModel::new(quad, vec![instance], device);

        pipeline_group.add_pipeline(
            vec![instance_model],
            include_str!("../shader/vs_0/3d_noise_geometry.wgsl"),
            device,
            config,
        )?;

        let quad = Model::new_quad(device);
        let q_instance = Instance::new();
        let q_instance_model = InstanceModel::new(quad, vec![q_instance], device);

        let disk = Model::new_disk(device, 200);
        let d_instance = Instance::new();
        let d_instance_model = InstanceModel::new(disk, vec![d_instance], device);

        pipeline_group.add_pipeline(
            vec![q_instance_model, d_instance_model],
            include_str!("../shader/vs_0/2d_noise.wgsl"),
            device,
            config,
        )?;

        let disk = Model::new_disk(device, 200);

        let instances = (0..NB_DISKS).map(|_| Instance::new()).collect();

        let instance_model = InstanceModel::new(disk, instances, device);

        pipeline_group.add_pipeline(
            vec![instance_model],
            include_str!("../shader/vs_0/2d_transparent.wgsl"),
            device,
            config,
        )?;

        let cube = Model::import(include_bytes!("../models/cube.obj"), device)?;
        let icosphere = Model::import(include_bytes!("../models/icosphere.obj"), device)?;
        let mf_room = Model::import(include_bytes!("../models/mfroom_3d.obj"), device)?;
        let pyramide = Model::import(include_bytes!("../models/pyramide.obj"), device)?;

        let instance = Instance::new();
        let cube = InstanceModel::new(cube, vec![instance], device);
        let instance = Instance::new();
        let icosphere = InstanceModel::new(icosphere, vec![instance], device);
        let instance = Instance::new();
        let mf_room = InstanceModel::new(mf_room, vec![instance], device);
        let instance = Instance::new();
        let pyramide = InstanceModel::new(pyramide, vec![instance], device);

        pipeline_group.add_pipeline(
            vec![cube, icosphere, mf_room, pyramide],
            include_str!("../shader/vs_0/3d.wgsl"),
            device,
            config,
        )?;

        let dyn_pipelines = vec![2, 3, 4, 5];
        for i in &dyn_pipelines {
            deactivate_pipeline(&mut pipeline_group.pipelines[*i]);
        }

        Ok(State {
            noise_3d_activated: false,
            noise_3d_start_time: 0.0,
            noise_3d_duration: 0.0,

            full_activated: (false, 0),
            full_start_time: 0.0,
            full_duration: 0.0,

            wf_3d_activated: (false, 0),
            wf_3d_start_time: 0.0,
            wf_3d_duration: 0.0,
            wf_3d_axis: [0.0, 1.0, 0.0].into(),

            disk_activated: [false; NB_DISKS],
            disk_start_time: [0.0; NB_DISKS],
            disk_duration: [0.0; NB_DISKS],
            disk_scale: [0.0; NB_DISKS],

            dyn_pipelines,
            active_pipelines: [3, 4, 5],
            rng: rand::thread_rng(),

            pipeline_switch_time: 0.0,
            show,
        })
    }

    pub fn switch_pipelines(&mut self, pipelines: &mut Vec<Pipeline>) {
        let i = (0..audio::NB_AUDIO_CHANNELS).choose(&mut self.rng).unwrap();
        let old_index = self.active_pipelines[i];
        deactivate_pipeline(&mut pipelines[old_index]);
        let mut candidate_pipelines = vec![];
        for i in &self.dyn_pipelines {
            let mut is_active = false;
            for j in &self.active_pipelines {
                if *i == *j {
                    is_active = true;
                    break;
                }
            }
            if !is_active {
                candidate_pipelines.push(*i);
            }
        }
        let new_index = candidate_pipelines.choose(&mut self.rng).unwrap();
        self.active_pipelines[i] = *new_index;
    }
    pub fn update(
        &mut self,
        pipelines: &mut Vec<Pipeline>,
        time: f32,
        old_audio: &audio::Data,
        new_audio: &audio::Data,
    ) {
        if time > self.pipeline_switch_time
            && self.dyn_pipelines.len() > self.active_pipelines.len()
        {
            self.pipeline_switch_time = get_switch_time(time, &mut self.rng);
            self.switch_pipelines(pipelines);
        }

        for (i, a) in self.active_pipelines.clone().iter().enumerate() {
            let o_a = old_audio.gain[i];
            let n_a = new_audio.gain[i];
            match a {
                2 => self.update_noise_3d(&mut pipelines[*a], time, o_a, n_a),
                3 => self.update_full(&mut pipelines[*a], time, o_a, n_a),
                4 => self.update_disk(&mut pipelines[*a], time, o_a, n_a),
                5 => self.update_wf_3d(&mut pipelines[*a], time, o_a, n_a),
                _ => unreachable!(),
            }
        }

        self.update_background_color(&mut pipelines[0], time);
    }

    fn update_background_color(&self, pipeline: &mut Pipeline, time: f32) {
        let bg = &mut pipeline.instance_models[0].instances[0];
        let pi = std::f64::consts::PI;
        let t: f64 = 2.0 * pi * time as f64 / COLOR_SHADING_PERIOD;
        let x = t.cos() as f32;

        for i in 0..4 {
            match self.show {
                Show::Lua => {
                    bg.color[i] = hex_to_f(COLOR_0_0[i]) * x + (1.0 - x) * hex_to_f(COLOR_4_0[i])
                }
                Show::MariusJulien => {
                    bg.color[i] = hex_to_f(COLOR_2_1[i]) * x + (1.0 - x) * hex_to_f(COLOR_4_1[i])
                }
            }
        }
    }

    fn update_noise_3d(
        &mut self,
        pipeline: &mut Pipeline,
        time: f32,
        old_audio: f32,
        new_audio: f32,
    ) {
        if new_audio > 1.5 && old_audio < 1.5 {
            self.activate_noise_3d(time, &mut pipeline.instance_models);
        }

        if self.noise_3d_activated {
            let t = time - self.noise_3d_start_time;
            if t > self.noise_3d_duration {
                self.noise_3d_activated = false;
                pipeline.instance_models[0].instances[0].position[0] = 0.0;
            }
        }
    }

    fn update_full(&mut self, pipeline: &mut Pipeline, time: f32, old_audio: f32, new_audio: f32) {
        if new_audio > 1.5 && old_audio < 1.5 {
            self.activate_full(time, &mut pipeline.instance_models);
        }

        if self.full_activated.0 {
            let t = time - self.full_start_time;
            if t > self.full_duration {
                self.full_activated.0 = false;
                pipeline.instance_models[self.full_activated.1].instances[0].scale = 0.0;
            }
        }
    }

    fn update_wf_3d(&mut self, pipeline: &mut Pipeline, time: f32, old_audio: f32, new_audio: f32) {
        if new_audio > 2.0 && old_audio < 2.0 {
            self.activate_wf_3d(time, &mut pipeline.instance_models);
        }

        if self.wf_3d_activated.0 {
            let i = &mut pipeline.instance_models[self.wf_3d_activated.1].instances[0];
            i.rotation = cgmath::Basis3::from_axis_angle(self.wf_3d_axis, cgmath::Rad(0.5 * time));
            let t = time - self.wf_3d_start_time;
            if t > self.wf_3d_duration {
                self.wf_3d_activated.0 = false;
                pipeline.instance_models[self.wf_3d_activated.1].instances[0].scale = 0.0;
            }
        }
    }

    fn activate_wf_3d(&mut self, time: f32, i_ms: &mut Vec<InstanceModel>) {
        let i = (0..i_ms.len()).choose(&mut self.rng).unwrap();
        self.wf_3d_activated = (true, i);
        self.wf_3d_start_time = time;
        self.wf_3d_duration = 3.0 * self.rng.gen::<f32>() + 3.0;
        self.wf_3d_axis = {
            let mut axis = cgmath::Vector3::<f32>::zero();
            while axis == cgmath::Vector3::<f32>::zero() {
                axis = cgmath::Vector3::<f32>::from([
                    self.rng.gen::<f32>(),
                    self.rng.gen::<f32>(),
                    self.rng.gen::<f32>(),
                ]);
            }
            let norm = (axis.x.powi(2) + axis.y.powi(2) + axis.z.powi(2)).sqrt();
            (1.0 / norm) * axis
        };

        for i_m in &mut *i_ms {
            i_m.instances[0].scale = 0.0;
        }

        let instance = &mut i_ms[i].instances[0];

        instance.color = match self.show {
            Lua => get_color_0(&mut self.rng),
            MariusJulien => get_color_1(&mut self.rng),
        };
        instance.scale = 1.0;
        instance.position = (
            0.5 - 1.0 * self.rng.gen::<f32>(),
            0.5 - 1.0 * self.rng.gen::<f32>(),
            0.0,
        )
            .into();
    }

    fn activate_noise_3d(&mut self, time: f32, i_ms: &mut Vec<InstanceModel>) {
        self.noise_3d_activated = true;
        self.noise_3d_start_time = time;
        self.noise_3d_duration = 1.0 * self.rng.gen::<f32>() + 1.0;

        let instance = &mut i_ms[0].instances[0];

        instance.color = match self.show {
            Lua => get_color_0(&mut self.rng),
            MariusJulien => get_color_1(&mut self.rng),
        };

        instance.scale = 1.0;
        instance.position[0] = 1.0;
        instance.position[1] = 0.5 - 1.0 * self.rng.gen::<f32>();
        instance.position[2] = 0.5 - 1.0 * self.rng.gen::<f32>();
    }

    fn activate_full(&mut self, time: f32, i_ms: &mut Vec<InstanceModel>) {
        let i = (0..i_ms.len()).choose(&mut self.rng).unwrap();
        self.full_activated = (true, i);
        self.full_start_time = time;
        self.full_duration = 0.6 * self.rng.gen::<f32>() + 0.4;

        for i_m in &mut *i_ms {
            i_m.instances[0].scale = 0.0;
        }

        let instance = &mut i_ms[i].instances[0];

        instance.color = match self.show {
            Lua => get_color_0(&mut self.rng),
            MariusJulien => get_color_1(&mut self.rng),
        };
        instance.scale = self.rng.gen::<f32>() * 0.1 + 0.1;
        instance.position = (
            0.5 - 1.0 * self.rng.gen::<f32>(),
            0.5 - 1.0 * self.rng.gen::<f32>(),
            0.0001 + 0.0008 * self.rng.gen::<f32>(),
        )
            .into();
    }

    fn update_disk(&mut self, pipeline: &mut Pipeline, time: f32, old_audio: f32, new_audio: f32) {
        let disks_i = &mut pipeline.instance_models[0].instances;

        if new_audio > 1.5 && old_audio < 1.5 {
            self.activate_disk(time, disks_i);
        }

        for i in 0..NB_DISKS {
            if self.disk_activated[i] {
                let t = time - self.disk_start_time[i];
                if t > self.disk_duration[i] {
                    self.disk_activated[i] = false;
                    disks_i[i].scale = 0.0;
                    continue;
                }
                disks_i[i].scale = self.disk_scale[i] + DISK_SPEED * t;
            }
        }
    }

    fn activate_disk(&mut self, time: f32, instances: &mut [Instance]) {
        for (i, d) in instances.iter_mut().enumerate().take(NB_DISKS) {
            if !self.disk_activated[i] {
                self.disk_activated[i] = true;
                d.color = match self.show {
                    Lua => get_color_0(&mut self.rng),
                    MariusJulien => get_color_1(&mut self.rng),
                };
                d.position = (
                    1.0 - 2.0 * self.rng.gen::<f32>(),
                    1.0 - 2.0 * self.rng.gen::<f32>(),
                    0.0,
                )
                    .into();
                self.disk_start_time[i] = time;
                self.disk_scale[i] = 0.1;
                self.disk_duration[i] = self.rng.gen::<f32>() + 0.5;
                return;
            }
        }
    }
}

fn hex_to_f(c: u8) -> f32 {
    c as f32 / 255.0
}
