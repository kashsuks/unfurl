<center><h1>Unfurl</h1></center>

[![Netlify Status](https://api.netlify.com/api/v1/badges/84a53e81-bf9b-4762-b1c7-4d11e476ee90/deploy-status)](https://app.netlify.com/projects/unfurl-rust/deploys)

Visualize JSON data easily right through your browser
<img width="1510" height="854" alt="image" src="https://github.com/user-attachments/assets/24f54800-6a59-4d19-a1a1-bd4293fc245a" />
<img width="1509" height="855" alt="image" src="https://github.com/user-attachments/assets/932a21e2-2eee-4eea-b425-9febb046b1f2" />
<img width="1512" height="853" alt="image" src="https://github.com/user-attachments/assets/3fd5cdcf-d4a3-4620-bcaa-342b8e617051" />

## Features
- Format a JSON file by dragging and dropping or pasting into the editor
- Compare two pieces of JSON data by using the `Diff` button in the top right
- Choose between the themes in the settings button in the top right
- Search through JSON data and count depth and nodes
- Hover over the arrow for any piece of data and get its path

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
