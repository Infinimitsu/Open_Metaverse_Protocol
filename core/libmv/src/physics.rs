use rapier3d::prelude::*;

pub struct PhysicsEngine {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    
    // Track the local player's body handle
    pub player_handle: Option<RigidBodyHandle>,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            player_handle: None,
        }
    }

    pub fn setup_player(&mut self) {
        // 1. Create a Dynamic Body (The Player)
        // Lock rotations so the capsule doesn't tip over
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 5.0, 0.0])
            .lock_rotations() 
            .build();
            
        // 2. Create a Collider (Capsule: height 1.0, radius 0.5)
        let collider = ColliderBuilder::capsule_y(0.5, 0.5)
            .restitution(0.0) // No bounce
            .build();
            
        // 3. Create a Floor (Static)
        let floor_body = RigidBodyBuilder::fixed().build();
        let floor_collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        
        // 4. Add to sets
        let floor_handle = self.rigid_body_set.insert(floor_body);
        self.collider_set.insert_with_parent(floor_collider, floor_handle, &mut self.rigid_body_set);

        let player_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(collider, player_handle, &mut self.rigid_body_set);

        self.player_handle = Some(player_handle);
    }

    pub fn step(&mut self, _dt: f32) {
        // Rapier steps a fixed timestep internally, usually handled automatically
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }

    pub fn set_player_velocity(&mut self, x: f32, z: f32) {
        if let Some(handle) = self.player_handle {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                // Keep existing Y velocity (gravity), update X/Z
                let current_linvel = body.linvel();
                body.set_linvel(vector![x, current_linvel.y, z], true);
            }
        }
    }

    // NEW: Jump Implementation
    pub fn jump(&mut self) {
        if let Some(handle) = self.player_handle {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                // Simple check: only jump if Y velocity is near zero (grounded-ish)
                // In production, we'd use a raycast to check for ground.
                if body.linvel().y.abs() < 0.1 {
                    body.apply_impulse(vector![0.0, 5.0, 0.0], true);
                }
            }
        }
    }

    pub fn get_player_position(&self) -> (f32, f32, f32) {
        if let Some(handle) = self.player_handle {
            if let Some(body) = self.rigid_body_set.get(handle) {
                let t = body.translation();
                return (t.x, t.y, t.z);
            }
        }
        (0.0, 0.0, 0.0)
    }
}