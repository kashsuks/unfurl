# unfurl
Visualize JSON data easily

## Local Development

Clone the repository via git
```bash
git clone https://github.com/kashsuks/unfurl.git
```

Change into the directory
```bash
cd unfurl
```

Once there, use cargo to run
```bash
cargo r
```
or 
```bash
cargo run
```

## Deploy to Web

Deploy the repository on a provider such as Vercel, Netlify, or GitHub Pages

Once done, run the following command at the root of the repository

```bash
trunk build --release --public-url /
```

Deploy whatever builds to the /dist
