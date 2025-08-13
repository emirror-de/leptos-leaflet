use leptos::logging::warn;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::core::JsStoredValue;

use super::LeafletMapContext;

/// Converts tile coordinates (x, y, z) to a quadkey string.
/// Based on Microsoft's QuadKey algorithm.
fn tile_to_quadkey(x: u32, y: u32, z: u32) -> String {
    let mut quadkey = String::new();
    
    for i in (1..=z).rev() {
        let mut digit = 0;
        let mask = 1 << (i - 1);
        
        if (x & mask) != 0 {
            digit += 1;
        }
        if (y & mask) != 0 {
            digit += 2;
        }
        
        quadkey.push(char::from_digit(digit, 10).unwrap_or('0'));
    }
    
    quadkey
}

/// A quad tile layer component that uses quadkey-based URLs.
/// Instead of the standard {z}/{x}/{y} pattern, this component
/// expects URLs with a {q} placeholder for the quadkey.
#[component(transparent)]
pub fn QuadTileLayer(
    #[prop(into)] url: String,
    #[prop(into, optional)] attribution: String,
    #[prop(optional)] bring_to_front: bool,
    #[prop(optional)] bring_to_back: bool,
    #[prop(default = 0.0)] min_zoom: f64,
    #[prop(default = 18.0)] max_zoom: f64,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");

    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            // Create tile layer options
            let options = leaflet::TileLayerOptions::default();
            if !attribution.is_empty() {
                options.set_attribution(attribution.to_string());
            }
            options.set_min_zoom(min_zoom);
            options.set_max_zoom(max_zoom);
            
            // Create a standard tile layer first
            let map_layer = leaflet::TileLayer::new_options(&url, &options);
            
            // Override the getTileUrl method to use quadkey
            let url_pattern = url.clone();
            let get_tile_url_closure = Closure::wrap(Box::new(move |coords: JsValue| -> String {
                // Extract x, y, z from coords object
                let x = js_sys::Reflect::get(&coords, &JsValue::from_str("x"))
                    .unwrap_or(JsValue::from(0))
                    .as_f64()
                    .unwrap_or(0.0) as u32;
                let y = js_sys::Reflect::get(&coords, &JsValue::from_str("y"))
                    .unwrap_or(JsValue::from(0))
                    .as_f64()
                    .unwrap_or(0.0) as u32;
                let z = js_sys::Reflect::get(&coords, &JsValue::from_str("z"))
                    .unwrap_or(JsValue::from(0))
                    .as_f64()
                    .unwrap_or(0.0) as u32;
                
                let quadkey = tile_to_quadkey(x, y, z);
                url_pattern.replace("{q}", &quadkey)
            }) as Box<dyn Fn(JsValue) -> String>);
            
            // Override the getTileUrl method on the layer instance
            js_sys::Reflect::set(
                &map_layer,
                &JsValue::from_str("getTileUrl"),
                get_tile_url_closure.as_ref().unchecked_ref(),
            ).unwrap();
            
            get_tile_url_closure.forget(); // Prevent cleanup
            
            map_layer.add_to(&map);

            match (bring_to_front, bring_to_back) {
                (true, true) => warn!("The parameters are set to bring the layer to front and back at the same time. Ignoring these parameters..."),
                (true, false) => {map_layer.bring_to_front();}
                (false, true) => {map_layer.bring_to_back();}
                (false, false) => (),
            }

            let map_layer = JsStoredValue::new_local(map_layer);

            on_cleanup(move || {
                map_layer.with_value(|v| v.remove());
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_to_quadkey() {
        // Test cases based on Microsoft's QuadKey documentation
        // https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system
        
        // Level 1 tests
        assert_eq!(tile_to_quadkey(0, 0, 1), "0");
        assert_eq!(tile_to_quadkey(1, 0, 1), "1");
        assert_eq!(tile_to_quadkey(0, 1, 1), "2");
        assert_eq!(tile_to_quadkey(1, 1, 1), "3");
        
        // Level 2 tests  
        assert_eq!(tile_to_quadkey(2, 1, 2), "12");
        assert_eq!(tile_to_quadkey(0, 2, 2), "20");
        
        // Level 3 test - example from Microsoft docs
        assert_eq!(tile_to_quadkey(3, 5, 3), "213");
        
        // Level 0 should return empty string
        assert_eq!(tile_to_quadkey(0, 0, 0), "");
    }
}