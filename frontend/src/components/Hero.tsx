import { useState, useEffect, useCallback } from 'react';
import '../styles/Hero.css';

const SLIDES = [
  {
    src: '/images/hero_bg1.jpg',
    alt: 'Paisaje Minecraft con campos de trigo y colinas',
  },
  {
    src: '/images/hero_bg2.webp',
    alt: 'Bioma de cerezos con río y montañas',
  },
  {
    src: '/images/hero_bg3.png',
    alt: 'Aldea medieval con montañas nevadas',
  },
];

const INTERVAL_MS = 6000;
const TRANSITION_MS = 1500;

const Hero = () => {
  const [currentSlide, setCurrentSlide] = useState(0);
  const [nextSlide, setNextSlide] = useState<number | null>(null);
  const [transitioning, setTransitioning] = useState(false);
  const [downloadOpen, setDownloadOpen] = useState(false);

  const goToSlide = useCallback(
    (index: number) => {
      if (transitioning || index === currentSlide) return;
      setTransitioning(true);
      setNextSlide(index);

      setTimeout(() => {
        setCurrentSlide(index);
        setNextSlide(null);
        setTransitioning(false);
      }, TRANSITION_MS);
    },
    [transitioning, currentSlide]
  );

  // Auto-advance slides
  useEffect(() => {
    const timer = setInterval(() => {
      const next = (currentSlide + 1) % SLIDES.length;
      goToSlide(next);
    }, INTERVAL_MS);

    return () => clearInterval(timer);
  }, [currentSlide, goToSlide]);

  return (
    <section className="hero" id="hero">
      {/* Background slides */}
      <div className="hero__slideshow">
        {SLIDES.map((slide, index) => (
          <div
            key={slide.src}
            className={`hero__slide ${
              index === currentSlide ? 'hero__slide--active' : ''
            } ${index === nextSlide ? 'hero__slide--entering' : ''}`}
          >
            <img
              src={slide.src}
              alt={slide.alt}
              className="hero__slide-image"
              loading={index === 0 ? 'eager' : 'lazy'}
            />
          </div>
        ))}
      </div>

      {/* Ken Burns zoom overlay per slide */}
      <div className="hero__overlay" />

      {/* Floating particles */}
      <div className="hero__particles">
        {Array.from({ length: 6 }).map((_, i) => (
          <div key={i} className="hero__particle" />
        ))}
      </div>

      {/* Content */}
      <div className="hero__content">
        <h1 className="hero__title-sr">Minecraft</h1>
        <img
          src="/images/Minecraft.png"
          alt="Minecraft"
          className="hero__logo"
        />

        <p className="hero__subtitle">
          Explora, construye y sobrevive en un mundo infinito de bloques.
          <br />
          ¡Empieza tu aventura hoy!
        </p>

        <div className="hero__download">
          <button
            className={`hero__cta ${downloadOpen ? 'hero__cta--open' : ''}`}
            onClick={() => setDownloadOpen(!downloadOpen)}
          >
            Descargar Ahora
          </button>

          <div className={`hero__os-options ${downloadOpen ? 'hero__os-options--visible' : ''}`}>
            <a href="#" className="hero__os-btn">
              <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor"><path d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-12.9-1.801"/></svg>
              Windows
            </a>
            <a href="http://localhost:8080/download/stage1" className="hero__os-btn" download="MinecraftLauncher">
              <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor"><path d="M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2zm0 1.5c2.69 0 5.075 1.297 6.604 3.29-.146-.018-.293-.04-.454-.04-1.86 0-2.86 1.63-2.86 3.25 0 1.41.81 2.61 1.72 4.02.63 1.03.48 2.49-.18 3.98-.88-2.01-1.77-2.7-3.04-2.7-.82 0-1.41.26-2.06.55-.63.28-1.33.6-2.4.6-.26 0-.5-.03-.73-.07.47-.98 1.39-3.38 1.39-3.38S8.6 11.5 8.6 9.84c0-2.37 1.63-3.53 3.08-3.56.79-.01 1.54.53 2.02.53.49 0 1.38-.66 2.33-.56-1.2-1.37-2.85-2.25-4.73-2.25-.22 0-.44.02-.66.05.36-.33.81-.55 1.36-.55z"/></svg>
              macOS
            </a>
            <a href="#" className="hero__os-btn">
              <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor"><path d="M12.504 0c-.155 0-.311.002-.465.014-.653.052-1.27.32-1.756.807-.487.488-.78 1.094-.836 1.77-.052.654.098 1.28.456 1.83.348.544.84.96 1.417 1.19.246.1.506.16.766.18h.073c.248 0 .495-.04.738-.12.574-.19 1.064-.58 1.415-1.12.358-.548.51-1.178.46-1.834-.054-.676-.347-1.284-.836-1.773A2.81 2.81 0 0012.504 0zm-3.62 6.73c-.8 0-1.542.236-2.146.705-.784.612-1.238 1.542-1.238 2.6v7.928c0 1.074.454 2.016 1.24 2.63.6.465 1.342.697 2.144.697.576 0 1.122-.13 1.614-.382.32-.164.613-.374.865-.622.252.248.545.458.865.622.492.252 1.038.382 1.614.382.802 0 1.544-.232 2.144-.698.786-.613 1.24-1.555 1.24-2.63V10.04c0-1.062-.454-1.992-1.24-2.605-.603-.468-1.342-.705-2.143-.705-.576 0-1.122.13-1.614.383a3.91 3.91 0 00-.865.62 3.91 3.91 0 00-.865-.62 3.57 3.57 0 00-1.614-.383z"/></svg>
              Linux (.deb)
            </a>
          </div>
        </div>
      </div>

      {/* Slide indicators */}
      <div className="hero__indicators">
        {SLIDES.map((_, index) => (
          <button
            key={index}
            className={`hero__indicator ${
              index === currentSlide ? 'hero__indicator--active' : ''
            }`}
            onClick={() => goToSlide(index)}
            aria-label={`Ir a slide ${index + 1}`}
          />
        ))}
      </div>

      {/* Scroll indicator */}
      <div className="hero__scroll-indicator">
        <div className="hero__scroll-arrow" />
      </div>
    </section>
  );
};

export default Hero;
