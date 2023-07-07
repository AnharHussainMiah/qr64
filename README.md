# QR64 - Quantum Simulator for Rust (WIP)

QR64 is an experimental quantum computing simulator for the Rust.
This code was adapted from [dakk's qc64](https://github.com/dakk/qc64/) and [Coding Nexus](https://github.com/webnexus-uk/NodeJS-quantum-simulator)

![Creating a Bell State in NodeJS](result.png)

## Features

- Simulate operations of various quantum gates including Pauli-X, Pauli-Y, Pauli-Z, Hadamard, CNOT, and SWAP on a two-qubit system.

## How it works

Starting from the initial state |00>, the simulator can apply various quantum gates to manipulate the state of the two-qubit system. This allows for the simulation of various quantum computing concepts!

## Gate Sequences and Outputs

The simulator supports various gate sequences, and the measurement results will vary based on the gate sequence used. Here are some examples:

### Example 1

**Gate Sequence**: x0,x1,y0,y1,z0,z1,h0,h1,cx,sw

**Measurement Results**:

| Quantum State | Measurement Count | Output                |
| ------------- | ----------------- | --------------------- |
| 00            | 5                 | QQQQQ                 |
| 01            | 0                 |                       |
| 10            | 0                 |                       |
| 11            | 23                | QQQQQQQQQQQQQQQQQQQQQ |

### Example 2

**Gate Sequence**: h0,h1,h0,h1,h0,h1

**Measurement Results**:

| Quantum State | Measurement Count | Output        |
| ------------- | ----------------- | ------------- |
| 00            | 14                | QQQQQQQQQQQQQ |
| 01            | 0                 |               |
| 10            | 0                 |               |
| 11            | 14                | QQQQQQQQQQQQQ |

## Contributions

Contributions to QR64 are welcome! Feel free to create a pull request or open an issue if you have ideas for improvements or spot any bugs.

## License

QR64 is released under MIT License.
