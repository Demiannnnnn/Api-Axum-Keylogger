import Navbar from './components/Navbar';
import Hero from './components/Hero';
import NewsSection from './components/NewsSection';
import Footer from './components/Footer';
import './styles/index.css';

function App() {
  return (
    <>
      <Navbar />
      <main>
        <Hero />
        <NewsSection />
      </main>
      <Footer />
    </>
  );
}

export default App;
