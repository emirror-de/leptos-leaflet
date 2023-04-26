mod evented;
mod grid_layer;
mod layer;
mod layer_group;
mod map;
pub mod plugins;
mod popup;
mod raster;
mod shapes;
mod tooltip;
mod div_overlay;

use js_sys::{Object, Array};
use wasm_bindgen::prelude::*;

pub use evented::Evented;
pub use grid_layer::{GridLayer, GridLayerOptions};
pub use layer::Layer;
pub use layer_group::LayerGroup;
pub use map::{LocateOptions, Map, MapOptions};
pub use popup::{Popup, PopupOptions};
pub use raster::{TileLayer, TileLayerOptions};
pub use shapes::{
    Circle, CircleMarker, Path, PathOptions, Polygon, Polyline, PolylineOptions, Rectangle,
};
pub use tooltip::{Tooltip, TooltipOptions};
pub use div_overlay::DivOverlay;

#[macro_export]
macro_rules! object_property_set {
    ($a:ident, $b:ty) => {
        pub fn $a(&mut self, val: $b) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($a)),
                &wasm_bindgen::JsValue::from(val),
            );
            let _ = r;
            self
        }
    };
    ($a:ident, $b:ident, $c:ty) => {
        pub fn $a(&mut self, val: $c) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from(val),
            );
            let _ = r;
            self
        }
    };
}

#[macro_export]
macro_rules! object_property_set_with {
    ($a:ident, $b:ident, $c:expr) => {
        pub fn $a(&mut self) -> &mut Self {
            let r = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from($c),
            );
            let _ = r;
            self
        }
    };
}

#[macro_export]
macro_rules! object_construtor {
    () => {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_mut)]
            let mut r = JsCast::unchecked_into(Object::new());
            r
        }
    };
}

// Doesn't inside the proc_macro :(
// #[macro_export]
// macro_rules! import_method {
//     ($js_type:ty, $method_name:ident, $return_type:ty, $($v:ident: $t:ty),*) => {
//         #[wasm_bindgen(method, js_name = $method_name)]
//         pub fn $method_name(this: &$js_type, $($v: $t),*) -> $return_type;
//     };
// }

#[wasm_bindgen]
extern "C" {

    // mapboxGl
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub type mapboxGL;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &JsValue) -> mapboxGL;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &mapboxGL, map: &Map);

    #[derive(Debug)]
    pub type Icon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &JsValue) -> Icon;

    // Point

    #[derive(Debug)]
    pub type Point;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(x: u32, y: u32) -> Point;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Point) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Point) -> u32;

    // LatLng

    #[derive(Debug, Default, Clone)]
    pub type LatLng;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat: f64, lng: f64) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LatLng) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LatLng) -> f64;

    #[wasm_bindgen(method)]
    pub fn distanceTo(this: &LatLng, otherLatLng: &LatLng) -> f64;

    // LatLngBounds

    #[derive(Debug)]
    pub type LatLngBounds;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(corner1: &LatLng, corner2: &LatLng) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getNorthEast(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getSouthWest(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn contains(this: &LatLngBounds, latlng: &LatLng) -> bool;

    // Marker

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Marker;

    // [`Marker`](https://leafletjs.com/reference-1.7.1.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> Marker;

    // [`Marker`](https://leafletjs.com/reference-1.7.1.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setIcon(this: &Marker, icon: &Icon);

    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Marker) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Marker, latlng: &LatLng);

    #[wasm_bindgen(method)]
    pub fn on(this: &Marker, event_name: &str, handler: &JsValue);

    // MouseEvent

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Event)]
    pub type MouseEvent;

    #[wasm_bindgen(method, getter)]
    pub fn latlng(this: &MouseEvent) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn originalEvent(this: &MouseEvent) -> web_sys::Event;

    // Event

    #[derive(Debug, Clone)]
    pub type Event;

    #[wasm_bindgen(method, getter)]
    pub fn target(this: &Event) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn sourceTarget(this: &Event) -> Object;

    // FeatureGroup

    /// [`FeatureGroup`](https://leafletjs.com/reference-1.7.1.html#featuregroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = LayerGroup)]
    pub type FeatureGroup;

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#featuregroup-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &FeatureGroup, style: &JsValue);

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &FeatureGroup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &FeatureGroup);

    /// [`getBounds`](https://leafletjs.com/reference-1.7.1.html#featuregroup-getbounds)
    #[wasm_bindgen(method)]
    pub fn getBounds(this: &FeatureGroup) -> LatLngBounds;

    // GeoJSON

    /// [`GeoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type GeoJSON;

    /// [`L.geoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson-l-geojson)
    #[wasm_bindgen(js_namespace = L)]
    pub fn geoJSON(geojson: &JsValue, options: &JsValue) -> GeoJSON;

    /// [`addData`](https://leafletjs.com/reference-1.7.1.html#geojson-adddata)
    #[wasm_bindgen(method)]
    pub fn addData(this: &GeoJSON, data: &JsValue);

    /// [`resetStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-resetstyle)
    #[wasm_bindgen(method)]
    pub fn resetStyle(this: &GeoJSON, layer: Option<&Layer>);

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &GeoJSON, style: &JsValue);

    // Control

    #[derive(Debug)]
    pub type Control;

    #[wasm_bindgen(js_namespace = L, static_method_of = Control)]
    pub fn extend(props: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Control, map: &Map);
}

impl Into<LatLng> for (f64, f64) {
    fn into(self) -> LatLng {
        LatLng::new(self.0, self.1)
    }
}

impl Into<LatLng> for [f64; 2] {
    fn into(self) -> LatLng {
        LatLng::new(self[0], self[1])
    }
}

impl Into<LatLngBounds> for (LatLng, LatLng) {
    fn into(self) -> LatLngBounds {
        LatLngBounds::new(&self.0, &self.1)
    }
}

pub fn to_lat_lng_array<T: Into<LatLng> + Copy>(lat_lngs: &[T]) -> Array {
    let array = Array::new();
    for &lat_lng in lat_lngs {
        array.push(&lat_lng.into());
    }
    array
}