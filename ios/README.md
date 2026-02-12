# Repo Links
- [iOS](https://github.com/SwiftDSG/gidence-mst-ios)
- [Processor](https://github.com/SwiftDSG/gidence-mst-processor)
- [Server](https://github.com/SwiftDSG/gidence-mst-server)

# MST Testing Scenarios
This is a template for us to run tests for our awesome application, before reading any further, let us assume that the processor will utilise all the functionality we provided for all the scenarios filled below. (this basically means that the processor will try its best to detect anything we have created)

## Camera Specification
- HikVision [DS-2CD1021G2-IU](https://www.tokopedia.com/find/ds-2cd1021g2-iu)
- 2.8 mm
- 2 MP Resolution

## Testing Scenario
*Fill in our testing scenario (e.g. the camera is placed in a tools workshop, the camera is placed on the top corner of the room at 4m in height)*

## Compliant Cases
- **Case 1**: A person is wearing a hard hat, safety vest, and mask. The program should classify this as **compliant**.
- **Case 2**: A person is wearing a hair net and mask (without a safety vest). The program should classify this as **compliant**.

## Non-Compliance Cases
- **Case 3**: A person is not wearing any PPE. The program should classify this as **non-compliant**.
- **Case 4**: A person is only wearing a mask. The program should classify this as **non-compliant**.
- **Case 5**: A person is only wearing a hard hat. The program should classify this as **non-compliant**.
- **Case 6**: A person is only wearing a hard hat and a mask. The program should classify this as **non-compliant** because the safety vest is missing.
- **Case 7**: A person is only wearing a safety vest. The program should classify this as **non-compliant**.
- **Case 8**: A person is only wearing a safety vest and mask. The program should classify this as **non-compliant** because the hard hat is missing.
- **Case 9**: A person is wearing a hard hat and a safety vest. The program should classify this as **non-compliant** because the mask is missing.
- **Case 10**: A person is wearing a hair net and a safety vest. The program should classify this as **non-compliant** because the hard hat is missing.
- **Case 11**: A person is wearing a hair net but no mask. The program should classify this as **non-compliant**.

## Test Results
### Scenario #1 (Example)
*Provide any details if needed (such as the condition of the room or the video footage)*
| Person # | Case # | Condition | Compliant? | Description | Success? |
| -------- | ------ | --------- | ---------- | ----------- | -------- |
| 1 | Case 1 | Standing still, distance ±3m | Y | - | Y |
| 2 | Case 2 | Moving towards the camera | Y | Started detecting around ±6m | Y |
| 3 | Case 3 | Standing still, distance ±7m | NONE | Not detected as a person | N |
| 4 | Case 4 | Standing still, distance ±5m | N | - | Y |

