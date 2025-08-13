use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
          <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
              <QuadTileLayer 
                  url="https://ecn.t0.tiles.virtualearth.net/tiles/r{q}.png?g=1" 
                  attribution="&copy; Microsoft Corporation"/>
              <Marker position=position!(51.505, -0.09) >
                  <Popup>
                      <strong>{"Testing QuadTileLayer with Bing Maps"}</strong>
                  </Popup>
              </Marker>
        </MapContainer>
    }
}
