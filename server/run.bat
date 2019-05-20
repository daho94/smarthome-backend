@ECHO OFF

systemfd --no-pid -s https::3000 -- cargo watch -x run