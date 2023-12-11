import { createRoot } from 'react-dom/client';
import MyButton from './components/button';

import init, { greet } from '../../client-brain/pkg/client_brain';

init().then(() => {
  greet('World!');
});

const root = createRoot(document.body);
root.render(<h2>Hello from React! <MyButton title="I'm a button!" /> </h2>);