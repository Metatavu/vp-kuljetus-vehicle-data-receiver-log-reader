# VP-Kuljetus Vehicle Data Receiver Log Reader

[VP-Kuljetus Vehicle Data Receiver](https://www.github.com/metatavu/vp-kuljetus-vehicle-data-receiver) can be configured to write all incoming [`AVLFrame`]s to a file.
This program reads those files and writes them as JSON files in a directory structure based on the timestamp of each record.
# Example
`vp-kuljetus-vehicle-data-receiver-log-reader input.txt`
# Output
```
- input
 ├── 0
    ├── {hour}:{minute}:{second}.{millisecond}.json
 ├── 1
    ├── {hour}:{minute}:{second}.{millisecond}.json
 |--- input.json
```