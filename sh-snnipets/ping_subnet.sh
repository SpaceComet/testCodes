#!/bin/sh

# Convert IP to decimal
ip_to_decimal() {
	ip=$1
	IFS='.'
	set -- $ip
	echo $((($1 << 24) + ($2 << 16) + ($3 << 8) + $4))
}

# Convert decimal to IP
decimal_to_ip() {
	decimal=$1
	echo "$(((decimal >> 24) & 255)).$(((decimal >> 16) & 255)).$(((decimal >> 8) & 255)).$((decimal & 255))"
}

# Ping all possible IPs the subnet
ping_subnet_ips() {
	subnet=$1
	IFS='/'
	set $subnet

	base_ip=$1
	prefix_length=$2

	base_decimal=$(ip_to_decimal "$base_ip")
	# subnet_size=$((2 ** (32 - prefix_length)))
	subnet_size=$(echo "2^(32 - $prefix_length)" | bc)

	i=0
	while [ "$i" -lt "$subnet_size" ]; do
		decimal_ip=$((base_decimal + i))
		ip=$(decimal_to_ip $decimal_ip)
		#echo "$ip"

		if ping -c 1 -W 1 "$ip" >/dev/null 2>&1; then
			printf "$ip\tUP\n"
		else
			printf "$ip\tDOWN\n"
		fi

		i=$((i + 1))
	done
}

if [ $# -eq 1 ]; then
	ping_subnet_ips "$1"
else
	echo "Usage: $0 <subnet>"
	echo "Example: $0 192.168.1.0/24"
fi
