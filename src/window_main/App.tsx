// tauri
import { open } from '@tauri-apps/plugin-dialog' ;

// sttk3
import './App.css' ;

export const App = () => {
  return (
    <div
      class='group-panel'
    >
      <button
        class='button-select-file'
        onClick={ async () => {
          const path: string | null = await open({
            multiple: false, 
            directory: false, 
          }) ;

          console.log(path) ;
        }}
      >
        Select File
      </button>
    </div>
  ) ;
} ;
