# ErgoX Keyboard

## BOM Summary

| Product                    | Count |
|----------------------------|-------|
| RP2040 type-c Board        | 2     |
| Switch                     | 82    |
| Switch Connector           | 82    |
| EC11 Encoder               | 2     |
| 1N4148WS T4                | 84    |
| Stabilizer 2U              | 6     |
| TRRS Cable                 | 1     |
| TRRS Connector             | 2     |
| 2.54-1*20 Connector Female | 4     |
| Switch Board               | 2     |
| Bottom Board               | 2     |
| M2 * 3mm Copper Column     | 16    |
| M2 * 8mm Copper Column     | 50    |
| M2 * 3mm Screw             | 132   |

Key caps:

| Key Cap Type  | Count |
|---------------|-------|
| 1U            | 68    |
| 1.25U         | 4     |
| 1.5U          | 2     |
| 1.75U         | 2     |
| 2U            | 4     |
| 2.25U         | 2     |
| EC11 knob cap | 2     |

## Layout

Edited using [Keyboard-Layout-Editor.com](https://keyboard-layout-editor.com/)

Layout files:

- for generate `vial.json`: [keyboard-layout-full-vial-12r7c.json](./keyboard-layout-full-vial-12r7c.json)
- for design PCB and Case: [keyboard-layout-left.json](./keyboard-layout-left.json)

## Case

Generated using [Plate & Case Builder](http://builder.swillkb.com/), and then edited
using [LibreCAD](https://github.com/LibreCAD/LibreCAD).

Files: [case](./case)

```plaintext
---------------------------- switch board
  | 8mm copper column
  |  ------------------- PCB
  |   | 3mm copper column
---------------------------- bottom board
```

## PCB

Designed using [JLC EDA](https://lceda.cn/editor).

PCB Project: [https://oshwhub.com/lqlklu/ergox](https://oshwhub.com/lqlklu/ergox)

