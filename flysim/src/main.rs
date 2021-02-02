use flysim::{Network, LayerTopology};

fn main() {
    // Topology inspired by https://www.youtube.com/watch?v=aircAruvnKk&list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi&index=1
    let _network = Network::random(&[
        LayerTopology { neurons: 28 * 28 },
        LayerTopology { neurons: 16 },
        LayerTopology { neurons: 16 },
        LayerTopology { neurons: 10 },
    ]);
}
