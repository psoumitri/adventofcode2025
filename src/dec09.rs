
use crate::utils;
use geo::{BoundingRect, Contains, Coord, Polygon, Rect};

pub fn solve( test: bool ) -> Result<(), String> {
    let input = utils::read_lines(test, "dec09")?;
    let red_tiles = input.iter()
        .filter_map(|l| { 
            let (x_str, y_str) = l.split_once(',')?;
            let x = x_str.parse::<f64>().ok()?;
            let y = y_str.parse::<f64>().ok()?;
            Some((x,y))
        }).collect::<Vec<_>>();
    calc_max_area(red_tiles);
    Ok(())
}

fn calc_max_area(red_tiles: Vec<(f64, f64)>) {
    let poly1 = Polygon::new(red_tiles.clone().into(), vec![]);
    // let in_polygon = |p: Point<f64>| -> bool { 
    //     poly1.contains(&p) || poly1.intersects(&p)
    // };

    let brect = poly1.bounding_rect().unwrap();

    let mut max_area = 0f64;
    for i in &red_tiles { 
        for j in &red_tiles { 
            if i == j { continue; }

            let tl = ((i.0).min(j.0), (i.1).min(j.1));
            let br = ((i.0).max(j.0), (i.1).max(j.1));

            let area = (br.1-tl.1+1f64)*(br.0-tl.0+1f64);
            if area <= max_area { continue; }

            let rect = Rect::new(
                Coord { x: tl.0, y: tl.1 },
                Coord { x: br.0, y: br.1 },
            );

            if !brect.contains(&rect) { continue; }
            if poly1.contains(&rect) {
                max_area = area;
            }

        }
    }
    println!("Largest rectangle area: {}", max_area);
}


#[cfg(test)]

mod tests { 
    //use super::*;

    #[test]
    fn test_basics() { }
}