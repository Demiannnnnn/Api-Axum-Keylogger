import '../styles/NewsSection.css';

interface NewsItem {
  id: number;
  category: string;
  title: string;
  description: string;
  date: string;
  badge?: string;
  emoji: string;
}

const newsItems: NewsItem[] = [
  {
    id: 1,
    category: 'Minecraft: Java Edition',
    title: 'La nueva actualización ya está aquí',
    description: 'Descubre nuevos biomas, mobs y bloques en la última actualización de Minecraft.',
    date: '27 Jun 2026',
    badge: 'Nuevo',
    emoji: '⛏️',
  },
  {
    id: 2,
    category: 'Minecraft Dungeons',
    title: 'Aventuras épicas te esperan',
    description: 'Explora mazmorras, derrota mobs y recoge botín legendario en Minecraft Dungeons.',
    date: '25 Jun 2026',
    emoji: '🗡️',
  },
  {
    id: 3,
    category: 'Comunidad',
    title: 'Creaciones increíbles de la comunidad',
    description: 'Mira las construcciones más impresionantes creadas por jugadores de todo el mundo.',
    date: '23 Jun 2026',
    emoji: '🏗️',
  },
  {
    id: 4,
    category: 'Marketplace',
    title: 'Nuevo contenido disponible',
    description: 'Descubre skins, mundos y packs de texturas creados por la comunidad.',
    date: '20 Jun 2026',
    badge: 'Destacado',
    emoji: '🎨',
  },
];

const NewsSection = () => {
  return (
    <section className="news" id="noticias">
      <div className="news__header">
        <h2 className="news__title">Noticias Destacadas</h2>
        <a href="#" className="news__explore-link">
          Explorar
          <span className="news__explore-arrow">→</span>
        </a>
      </div>

      <div className="news__grid">
        {newsItems.map((item) => (
          <article key={item.id} className="news-card">
            <div className="news-card__image-wrapper">
              <div className="news-card__placeholder">
                {item.emoji}
              </div>
              {item.badge && (
                <span className="news-card__badge">{item.badge}</span>
              )}
            </div>
            <div className="news-card__body">
              <span className="news-card__category">{item.category}</span>
              <h3 className="news-card__title">{item.title}</h3>
              <p className="news-card__description">{item.description}</p>
              <time className="news-card__date">{item.date}</time>
            </div>
          </article>
        ))}
      </div>
    </section>
  );
};

export default NewsSection;
