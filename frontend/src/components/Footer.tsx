import '../styles/Footer.css';

const Footer = () => {
  const columns = [
    {
      title: 'Juegos',
      links: ['Minecraft: Java & Bedrock', 'Minecraft Dungeons', 'Minecraft Legends', 'Minecraft Education'],
    },
    {
      title: 'Comunidad',
      links: ['Página principal', 'Foros', 'Servidores', 'Feedback'],
    },
    {
      title: 'Soporte',
      links: ['Centro de ayuda', 'Estado de los servidores', 'Contacto', 'Reportar un bug'],
    },
    {
      title: 'Más',
      links: ['Merch', 'Redstone', 'Mojang Studios', 'Carreras'],
    },
  ];

  return (
    <footer className="footer">
      <div className="footer__content">
        {columns.map((col) => (
          <div key={col.title} className="footer__column">
            <h3 className="footer__column-title">{col.title}</h3>
            <ul className="footer__links">
              {col.links.map((link) => (
                <li key={link}>
                  <a href="#" className="footer__link">{link}</a>
                </li>
              ))}
            </ul>
          </div>
        ))}
      </div>

      <div className="footer__bottom">
        <span className="footer__copyright">
          © 2026 Mojang AB. Replica educativa.
        </span>
        <div className="footer__legal-links">
          <a href="#" className="footer__legal-link">Términos</a>
          <a href="#" className="footer__legal-link">Privacidad</a>
          <a href="#" className="footer__legal-link">Cookies</a>
          <a href="#" className="footer__legal-link">Accesibilidad</a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
