# VP-Kuljetus Vehicle Data Receiver Log Reader

[VP-Kuljetus Vehicle Data Receiver](https://www.github.com/metatavu/vp-kuljetus-vehicle-data-receiver) can be configured to write all incoming Teltonika packets to a file.
This program reads those files and writes them as JSON files in a directory structure based on the timestamp of each record.

This should be saved into `/usr/local/bin/vp-kuljetus-vehicle-data-receiver-log-reader` and made executable with `chmod +x /usr/local/bin/vp-kuljetus-vehicle-data-receiver-log-reader`.

```shell
#!/bin/sh

# Following script can be used to read the logs directly from the current Kubernetes pod (assuming there's only one instance)
# This script should be saved to /usr/local/bin/vp-logs.sh and made executable with chmod +x /usr/local/bin/vp-logs.sh
# Usage: <vp-logs IMEI> <namespace:defaults to staging>
#!/bin/sh

if [ -z "$1" ]
  then
    echo "No IMEI supplied!"
    echo "Usage <vp-logs IMEI>"
    exit
fi

IMEI=$1
DATE=$2
NAMESPACE=$3

if [ -z "$3" ]
  then
    NAMESPACE="staging"
fi

if [ -z "$2" ]
  then
    DATE=$(date +%F)
fi

POD_NAME=$(kubectl -n $NAMESPACE get pods -o=json | jq '. | .items[] | .metadata.name | select(startswith("transport-management-vehicle-data-receiver"))' | sed -E 's/\"//g')

kubectl -n $NAMESPACE exec -it $POD_NAME -- cat /opt/telematic-data/$IMEI/$DATE.txt | vp-kuljetus-vehicle-data-receiver-log-reader

code vp-kuljetus-logs/
```