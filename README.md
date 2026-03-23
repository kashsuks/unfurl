<center><h1>Unfurl</h1></center>

[![Netlify Status](https://api.netlify.com/api/v1/badges/84a53e81-bf9b-4762-b1c7-4d11e476ee90/deploy-status)](https://app.netlify.com/projects/unfurl-rust/deploys)

Visualize JSON data easily right through your browser
<img width="3024" height="1834" alt="image" src="https://github.com/user-attachments/assets/d025199e-dcb6-4a3a-9219-29581193d05c" />

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
