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

        <button className="hero__cta">Descargar Ahora</button>
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
