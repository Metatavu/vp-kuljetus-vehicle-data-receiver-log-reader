# VP-Kuljetus Vehicle Data Receiver Log Reader

[VP-Kuljetus Vehicle Data Receiver](https://www.github.com/metatavu/vp-kuljetus-vehicle-data-receiver) can be configured to write all incoming [`AVLFrame`]s to a file.
This program reads those files and writes them into a JSON file.
## Example
`vp-kuljetus-vehicle-data-receiver-log-reader input.txt` outputs `input.json`