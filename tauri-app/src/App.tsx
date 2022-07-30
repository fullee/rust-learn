import {useEffect, useState} from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import 'antd/dist/antd.css';
import {invoke} from "@tauri-apps/api";
import {Button} from "antd";

function App() {
  const [count, setCount] = useState(0)
  const [title, setTitle] = useState<string>('')
  useEffect(() => {
    invoke('greet', {name: "lwl"})
      .then((resp) => setTitle(String(resp)))
  }, [])

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo"/>
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo"/>
        </a>
      </div>
      <h1>Vite + React + {title}</h1>
      <div className="card">
        <Button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </Button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
        <Button onClick={async () => {
          const resp: string = await invoke('cvr', {})

          if (resp) {
            setTitle(resp);
            alert("读卡完成")
          } else {
            setTitle('');
            alert("请重新放置身份证")
          }
        }}>打开
        </Button>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
