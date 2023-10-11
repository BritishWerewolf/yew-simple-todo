#!/bin/bash
NODE_ENV=production npx tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
trunk serve
