import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import Heading from '@theme/Heading';
import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/">
            Get Started 📦
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title}`}
      description="Stel Docs & Guides!">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <section style={{marginTop: '3rem', textAlign: 'center'}}>
          <div style={{margin: '2rem 0'}}>
            <h3>See StelLang in Action!</h3>
            <div style={{width: '100%', maxWidth: '900px', margin: '0 auto'}}>
              {/* Replace the src below with your YouTube video link! */}
              <iframe
                width="100%"
                height="500"
                src="https://www.youtube.com/embed/PLACEHOLDER"
                title="StelLang Demo"
                frameBorder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowFullScreen
                style={{borderRadius: '16px', border: '2px solid #eee'}}
              ></iframe>
            </div>
            <p style={{fontStyle: 'italic', color: '#888'}}>Video coming soon! Stay tuned for a live demo of StelLang.</p>
          </div>
        </section>
      </main>
    </Layout>
  );
}
