install:
	cp target/release/service ~/.local/bin/psu-service -fv
	cp target/release/cli ~/.local/bin/psu-cli -fv
	chmod +x ~/.local/bin/psu-service
	chmod +x ~/.local/bin/psu-cli

service:
	mkdir -p ~/.local/share/systemd/user/
	cp systemd/psu-service.service ~/.local/share/systemd/user/
	systemctl --user daemon-reload


monitor:
	journalctl --user -u psu-service -f
