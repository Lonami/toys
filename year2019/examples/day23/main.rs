use year2019::intcode::{Program, StepResult};

const COMPUTER_COUNT: usize = 50;
const OUTPUT_ADDR: usize = 255;

struct Computer {
    program: Program,
    address: usize,
    state: PacketState,
    packet: Packet
}

#[derive(Default)]
struct Packet {
    dest: usize,
    x: i64,
    y: i64
}

enum PacketState {
    WaitDest,
    WaitX,
    WaitY,
}

impl Packet {
    fn new() -> Self {
        Self::default()
    }
}

struct ComputerNet {
    computers: Vec<Computer>
}

impl Computer {
    fn new(mut program: Program, address: usize) -> Self {
        program.set_stdin(vec![address as i64]);
        Self {
            program,
            address,
            state: PacketState::WaitDest,
            packet: Packet::new()
        }
    }

    fn feed_packet(&mut self, packet: Packet) {
        assert_eq!(self.address, packet.dest);
        self.program.push_input(packet.x);
        self.program.push_input(packet.y);
    }

    fn run(&mut self) -> Option<Packet> {
        loop {
            match self.program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => {
                    self.program.push_input(-1);
                    return None;
                },
                StepResult::Output(value) => {
                    match self.state {
                        PacketState::WaitDest => {
                            self.packet.dest = value as usize;
                            self.state = PacketState::WaitX;
                        },
                        PacketState::WaitX => {
                            self.packet.x = value;
                            self.state = PacketState::WaitY;
                        },
                        PacketState::WaitY => {
                            self.packet.y = value;
                            self.state = PacketState::WaitDest;
                            return Some(std::mem::take(&mut self.packet));
                        },
                    }
                },
                StepResult::CaughtFire => panic!("don't have fire extinguisher")
            }
        }
    }
}

impl ComputerNet {
    fn new(computers: Vec<Computer>) -> Self {
        Self { computers }
    }

    fn listen_y(&mut self, addr: usize) -> i64 {
        let mut packets = Vec::new();
        loop {
            packets.clear();
            for computer in self.computers.iter_mut() {
                if let Some(packet) = computer.run() {
                    packets.push(packet);
                }
            }

            while let Some(packet) = packets.pop() {
                if packet.dest == addr {
                    return packet.y;
                }
                self.computers[packet.dest].feed_packet(packet);
            }
        }
    }

    fn handle_nat(&mut self) -> i64 {
        let mut packets = Vec::new();
        let mut nat = Packet::default();
        let mut last_y = -1;
        loop {
            packets.clear();
            for computer in self.computers.iter_mut() {
                if let Some(packet) = computer.run() {
                    packets.push(packet);
                }
            }

            if packets.is_empty() {
                // All idle
                if nat.y == last_y {
                    return nat.y;
                }
                nat.dest = 0;
                last_y = nat.y;
                self.computers[0].feed_packet(std::mem::take(&mut nat));
            } else {
                while let Some(packet) = packets.pop() {
                    if packet.dest == 255 {
                        nat = packet;
                    } else {
                        self.computers[packet.dest].feed_packet(packet);
                    }
                }
            }
        }
    }
}

fn main() {
    let program = Program::from_stdin();
    let mut network = ComputerNet::new((0..COMPUTER_COUNT)
        .map(|addr| Computer::new(program.clone(), addr))
        .collect());

    println!("{}", network.listen_y(OUTPUT_ADDR));

    let mut network = ComputerNet::new((0..COMPUTER_COUNT)
        .map(|addr| Computer::new(program.clone(), addr))
        .collect());

    println!("{}", network.handle_nat());
}
