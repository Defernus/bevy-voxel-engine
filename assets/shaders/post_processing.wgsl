#import bevy_pbr::mesh_view_bindings

[[group(1), binding(0)]]
var texture: texture_2d<f32>;

[[group(1), binding(1)]]
var our_sampler: sampler;

[[stage(fragment)]]
fn fragment([[builtin(position)]] position: vec4<f32>) -> [[location(0)]] vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);

    // Sample each color channel with an arbitrary shift
    var output_color = textureSample(texture, our_sampler, uv);

    return output_color;
}
