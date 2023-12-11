import { createRoot } from 'react-dom/client';
import MyButton from './components/button';

const root = createRoot(document.body);
root.render(<h2>Hello from React! <MyButton title="I'm a button!" /> </h2>);