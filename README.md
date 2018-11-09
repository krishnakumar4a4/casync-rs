# casync-rs
Experimental: Pure rust implementation of casync https://github.com/systemd/casync

#### Very minimal implementation of casync tool.

## Command Usage
#### `input` file will be chunked, ``index.caidx`` and ``default.castr`` is created with chunks.
`casync-rs make --file input`

#### Expects `index.caidx` and `default.castr` directory to be present to construct `out` file from the chunks and index.
`casync-rs extract --file out`

#### Expects `default.castr` present with chunks corresponding to index.caidx file to construct the `out` file.
`casync-rs extract -i index.caidx  --file out`

#### Created `default.castr` directory and download chunks to it, followed by construction of `out` file from the given `index.caidx` file. 
`casync-rs extract -i index.caidx -s http://0.0.0.0:8000/ --file out`

#### Creates `out` file from `/index.caidx` and `/default.castr` endpoint from remote store to construct `out` file.
`casync-rs extract -i http://0.0.0.0:8000/index.caidx -s http://0.0.0.0:8000/ --file out`

#### Default minimum chunk size: 512KB and maximum chunk size: 1024KB

## Test scenario
- Run ``python -m http_server`` which runs http server on 8000 port
- In a different folder run ``--extract`` to create artifact. store and index file are also created in the same folder
