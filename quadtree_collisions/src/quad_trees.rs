use crate::physics::Physics;
use crate::{X_EXTENT, Y_EXTENT};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const ITEM_PER_QUAD: usize = 100;

pub struct QuadtreePlugin;
impl Plugin for QuadtreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Quadtree::new(Rect::default(), ITEM_PER_QUAD))
            .add_systems(PreUpdate, (clean_quadtree, update_quadtree_system))
            .add_systems(Update, draw_quadtree);
    }
}

#[derive(Debug, Clone)]
pub struct QuadtreeItem {
    entity: Entity,
    transform: Transform,
    physics: Physics,
}

#[derive(Component)]
pub struct QuadTreeDetect;

#[derive(Resource, Debug)]
pub struct Quadtree {
    bounds: Rect,                         // Define the bounds of this node
    capacity: usize,                      // Maximum number of items before splitting
    items: Vec<QuadtreeItem>,             // Items stored in this node
    children: Option<[Box<Quadtree>; 4]>, // Child quadtrees
}

impl Quadtree {
    fn new(bounds: Rect, capacity: usize) -> Self {
        Self {
            bounds,
            capacity,
            items: Vec::new(),
            children: None,
        }
    }

    // Method to insert an entity into the quadtree
    pub fn insert(&mut self, entity: Entity, transform: Transform) {
        if !self.point_in_bounds(transform.translation) {
            println!("shouldnt_run");
            // Position is outside the bounds of this quadtree node
            return;
        }

        if self.children.is_some() {
            // Recursively insert into the correct child
            let index = self.get_quadrant_index_for_position(transform.translation);
            if let Some(children) = &mut self.children {
                children[index].insert(entity, transform);
            }
        } else {
            // If we're at a leaf and below capacity, just add the item
            if self.items.len() < self.capacity {
                self.items.push(QuadtreeItem { entity, transform });
            } else {
                // Otherwise, we need to split and then insert
                self.split();
                self.insert(entity, transform); // Retry insertion after splitting
            }
        }
    }

    // Assume we have a method to check if a point belongs to a quadrant
    fn point_in_bounds(&self, position: Vec3) -> bool {
        // Check if the x coordinate of the position is within the bounds
        let in_x_bounds = position.x >= self.bounds.min.x && position.x <= self.bounds.max.x;
        // Check if the y coordinate of the position is within the bounds
        let in_y_bounds = position.y >= self.bounds.min.y && position.y <= self.bounds.max.y;

        // The point is within the bounds if both x and y coordinates are within their respective bounds
        in_x_bounds && in_y_bounds
    }

    fn get_quadrant_index_for_position(&self, position: Vec3) -> usize {
        let midpoint_x = self.bounds.min.x + self.bounds.width() / 2.0;
        let midpoint_y = self.bounds.min.y + self.bounds.height() / 2.0;

        // Determine the position relative to the midpoint
        let is_top_half = position.y > midpoint_y;
        let is_right_half = position.x > midpoint_x;

        // Determine the quadrant based on the position relative to the midpoint
        match (is_top_half, is_right_half) {
            (true, false) => 0,  // Top-left quadrant
            (false, false) => 1, // Bottom-left quadrant
            (false, true) => 2,  // Bottom-right quadrant
            (true, true) => 3,   // Top-right quadrant
                                  // This case should not happen as all possibilities are covered, but Rust requires exhaustiveness
        }
    }

    fn split(&mut self) {
        let mid_x = self.bounds.min.x + self.bounds.width() / 2.0;
        let mid_y = self.bounds.min.y + self.bounds.height() / 2.0;

        // Create and assign the four child quads based on the midpoint
        self.children = Some([
            Box::new(Quadtree::new(
                Rect::new(self.bounds.min.x, mid_y, mid_x, self.bounds.max.y),
                self.capacity,
            )),
            Box::new(Quadtree::new(
                Rect::new(self.bounds.min.x, self.bounds.min.y, mid_x, mid_y),
                self.capacity,
            )),
            Box::new(Quadtree::new(
                Rect::new(mid_x, self.bounds.min.y, self.bounds.max.x, mid_y),
                self.capacity,
            )),
            Box::new(Quadtree::new(
                Rect::new(mid_x, mid_y, self.bounds.max.x, self.bounds.max.y),
                self.capacity,
            )),
        ]);

        let length = self.items.len();
        for _ in 0..length {
            let item = self.items.pop().unwrap();
            let index = self.get_quadrant_index_for_position(item.transform.translation);
            if let Some(children) = &mut self.children {
                children[index].insert(item.entity, item.transform);
                return;
            }
        }
    }

    // Method to query the quadtree and find entities within a certain area
    // Additional methods like split, update, etc.
    pub fn query(&self, position: Vec3) -> Vec<QuadtreeItem> {
        if self.children.is_some() {
            let index = self.get_quadrant_index_for_position(position);
            if let Some(children) = &self.children {
                return children[index].query(position);
            }
        }
        self.items.clone()
    }
}

// Bevy system to update the quadtree
fn update_quadtree_system(
    mut quadtree: ResMut<Quadtree>,
    query: Query<(Entity, &Transform), With<QuadTreeDetect>>,
) {
    *quadtree = Quadtree::new(
        Rect::new(-X_EXTENT, -Y_EXTENT, X_EXTENT, Y_EXTENT),
        ITEM_PER_QUAD,
    );

    for (entity, transform) in query.iter() {
        quadtree.insert(entity, *transform)
    }
}

#[derive(Component, Debug)]
struct QuadTreeLine;

fn draw_quadtree(
    mut commands: Commands,
    quadtree: Res<Quadtree>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    draw_quadtree_node(&mut commands, &quadtree, &mut meshes, &mut materials);
}

fn draw_quadtree_node(
    commands: &mut Commands,
    node: &Quadtree,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let color = Color::WHITE;
    let outline_thickness = 0.2; // Thickness of the outline

    // Coordinates for the corners of the quadtree node
    let min_x = node.bounds.min.x;
    let max_x = node.bounds.max.x;
    let min_y = node.bounds.min.y;
    let max_y = node.bounds.max.y;
    let width = node.bounds.width();
    let height = node.bounds.height();

    // Create mesh handles for lines (outlines)
    let mut create_outline = |w: f32, h: f32, x: f32, y: f32| {
        let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(w, h)));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            QuadTreeLine,
        ));
    };

    // Top border
    create_outline(
        width,
        outline_thickness,
        (min_x + max_x) / 2.0,
        max_y - outline_thickness / 2.0,
    );
    // Bottom border
    create_outline(
        width,
        outline_thickness,
        (min_x + max_x) / 2.0,
        min_y + outline_thickness / 2.0,
    );
    // Left border
    create_outline(
        outline_thickness,
        height,
        min_x + outline_thickness / 2.0,
        (min_y + max_y) / 2.0,
    );
    // Right border
    create_outline(
        outline_thickness,
        height,
        max_x - outline_thickness / 2.0,
        (min_y + max_y) / 2.0,
    );

    // Recursively draw children if they exist
    if let Some(children) = &node.children {
        for child in children.iter() {
            draw_quadtree_node(commands, child, meshes, materials);
        }
    }
}

fn clean_quadtree(mut commands: Commands, query: Query<Entity, With<QuadTreeLine>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
