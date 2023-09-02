import {BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import { CurrencyListProvider } from './context/CurrencyContext';
import Assets from './pages/Assets';
import Summary from './pages/Summary';

function App() {

  return (
    <CurrencyListProvider>
      <Router>
        <Routes>
          <Route element={<Assets />} path="/"/>
          <Route element={<Summary />} path="/summary"/>
        </Routes>
      </Router>
    </CurrencyListProvider>
  );
}

export default App;
