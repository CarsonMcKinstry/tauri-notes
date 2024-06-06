import * as fs from 'fs';
import * as path from 'path';

import { dirname } from 'path';
import { fileURLToPath } from 'url';
    
const __dirname = dirname(fileURLToPath(import.meta.url));

function main() {
    const binPath = path.join(__dirname, './src-tauri/src/bin');
    const tempPath = path.join(__dirname, './src-tauri/temp');
    fs.renameSync(
        tempPath,
        binPath
    );
}

main();