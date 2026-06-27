import { useState, useEffect } from 'react';
import '../styles/Navbar.css';

const Navbar = () => {
  const [scrolled, setScrolled] = useState(false);
  const [mobileOpen, setMobileOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 50);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const navLinks = [
    { label: 'Juego', hasDropdown: true },
    { label: 'Tienda', hasDropdown: false },
    { label: 'Comunidad', hasDropdown: true },
    { label: 'Noticias', hasDropdown: true },
    { label: 'Actualizaciones', hasDropdown: false },
  ];

  return (
    <nav className={`navbar ${scrolled ? 'scrolled' : ''}`}>
      <div className="navbar__left">
        <a href="#" className="navbar__logo" aria-label="Minecraft Home">
          <img
            src="/images/logo.webp"
            alt="Minecraft"
            className="navbar__logo-icon"
          />
        </a>

        <button
          className={`navbar__mobile-toggle ${mobileOpen ? 'active' : ''}`}
          onClick={() => setMobileOpen(!mobileOpen)}
          aria-label="Toggle menu"
        >
          <span />
          <span />
          <span />
        </button>
      </div>

      <ul className={`navbar__nav ${mobileOpen ? 'open' : ''}`}>
        {navLinks.map((link) => (
          <li key={link.label}>
            <a href="#" className="navbar__link">
              {link.label}
              {link.hasDropdown && <span className="navbar__link-arrow">▼</span>}
            </a>
          </li>
        ))}
      </ul>

      <div className="navbar__right">
        <button className="navbar__icon-btn" aria-label="Buscar">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.35-4.35" />
          </svg>
        </button>

        <button className="navbar__icon-btn" aria-label="Usuario">
          <div className="pixel-user-icon">
            <div className="pixel-user-icon__face">
              <div className="pixel-user-icon__eyes">
                <div className="pixel-user-icon__eye" />
                <div className="pixel-user-icon__eye" />
              </div>
            </div>
          </div>
        </button>

        <button className="navbar__cta">
          Comprar Minecraft
        </button>
      </div>
    </nav>
  );
};

export default Navbar;
