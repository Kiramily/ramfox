build() {
	cargo build --release
}

install() {
	sudo cp ./target/release/ramfox /usr/bin/ramfox
	sudo cp ./scripts/ramfox.service /etc/systemd/user/ramfox.service
}

if ! command -v cargo &>/dev/null; then
	echo "cargo could not be found"
	exit 1
fi

while IFS= read -r -d '' dir; do
	directories+=("$dir")
done < <(find ~/.mozilla/firefox -mindepth 1 -maxdepth 1 -type d ! \( -name "static-*" -o -name "Crash Reports" -o -name "Pending Pings" \) -print0)

echo "select a profile"

for ((i = 0; i < ${#directories[@]}; i++)); do
	dir_name=$(basename "${directories[$i]}")
	printf "%d. %s\n" "$i" "$dir_name"
done

while true; do
	read -p "Select a profile: " idx
	if [[ $idx -ge 0 && $idx -le ${#directories[@]} ]]; then

		profile=$(basename ${directories[$idx]})

		echo """
[profile]
name = \""$profile"\"
auto-start = false
bin = \"/usr/bin/firefox\"

[profile.sync]
every = \""5mins"\"
""" >~/.config/ramfox.toml

		if [ -f "Cargo.toml" ]; then
			build
		elif [ -f "../Cargo.toml" ]; then
			cd ..
			build

		else
			git clone https://github.com/Kiramily/ramfox.git
			cd ramfox
			build
		fi

		install

		echo "installed"
		echo "You can now enable the ramfox service"
		echo "systemctl --user enable ramfox"
		echo "systemctl --user start ramfox"

		break
	else
		echo "Please use numbers to select a firefox profile"
	fi
done
