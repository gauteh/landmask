#![feature(test)]
extern crate test;

use std::io;
use std::fs::File;
use wkb;
use geo::*;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::algorithm::contains::Contains;
pub use rstar::{RTree, RTreeObject, AABB, PointDistance};
use rayon::prelude::*;

mod tests;

pub struct PolyWrapper(geo::Polygon<f64>);

impl RTreeObject for PolyWrapper
{
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let r = self.0.bounding_rect().unwrap();
        AABB::from_corners([r.min.x, r.min.y], [r.max.x, r.max.y])
    }
}

impl PointDistance for PolyWrapper
{
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let p = geo::Point::new(point[0], point[1]);
        p.euclidean_distance(&self.0)
    }

    fn contains_point(&self, point: &[f64; 2]) -> bool {
        let p = geo::Point::new(point[0], point[1]);
        self.0.contains(&p)
    }
}

pub fn contains<T>(tree: &RTree<T>, x: f64, y: f64) -> bool
where T: RTreeObject<Envelope = AABB<[f64; 2]>> + PointDistance
{
    let pnt: [f64; 2] = [x, y];
    let poly = tree.locate_at_point(&pnt);

    match poly {
        Some(_) => true,
        _       => false
    }
}

pub fn contains_many(tree: &RTree<PolyWrapper>, x: &[f64], y: &[f64])
    -> Vec<bool>
{
    x.par_iter()
     .zip(y)
     .map (|(xx, yy)| contains(&tree, *xx, *yy))
     .collect()
}

pub fn make_rtree_wkb(file: &str) -> io::Result<RTree<PolyWrapper>>
{
    println! ("opening wkb: {}", file);

    let mut f = File::open(file)?;
    let geom = wkb::wkb_to_geom(&mut f).unwrap();

    println! ("creating rtree");
    let tree = if let Geometry::MultiPolygon(mp) = geom {
        RTree::bulk_load(mp.into_iter().map(|p| PolyWrapper(p)).collect())
    } else {
        panic! ("Could not build RTree.")
    };

    Ok(tree)
}

