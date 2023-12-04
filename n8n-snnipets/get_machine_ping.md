Webhook that listens to `/webhook/ping`.
Reads the file `/files/machines-test.txt` that contains a list of machines.
Then a node will go through the list and ping the machines
The respond will be the entire list and a boolean value indicating if the machine is pingable or not.
