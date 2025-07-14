import React, { useState, useEffect } from 'react';
import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import styles from './registry.module.css';

const REGISTRY_URL = 'https://stellang.maheshdhingra.xyz/registry';

function PackageCard({ pkg }) {
  return (
    <div className={styles.packageCard}>
      <div className={styles.packageHeader}>
        <h3 className={styles.packageName}>
          <Link to={`/registry/package/${pkg.name}`}>
            {pkg.name}
          </Link>
        </h3>
        <span className={styles.packageVersion}>v{pkg.version}</span>
      </div>
      {pkg.description && (
        <p className={styles.packageDescription}>{pkg.description}</p>
      )}
      {pkg.authors && pkg.authors.length > 0 && (
        <p className={styles.packageAuthors}>
          by {pkg.authors.join(', ')}
        </p>
      )}
      <div className={styles.packageMeta}>
        <span className={styles.packageDate}>
          {new Date(pkg.upload_date).toLocaleDateString()}
        </span>
        <span className={styles.packageSize}>
          {(pkg.size / 1024).toFixed(1)} KB
        </span>
      </div>
    </div>
  );
}

function SearchBar({ onSearch }) {
  const [query, setQuery] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    onSearch(query);
  };

  return (
    <form onSubmit={handleSubmit} className={styles.searchForm}>
      <div className={styles.searchContainer}>
        <input
          type="text"
          placeholder="Search packages..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          className={styles.searchInput}
        />
                 <button type="submit" className={styles.searchButton}>
           Search
         </button>
      </div>
    </form>
  );
}

function RegistryHeader() {
  const { siteConfig } = useDocusaurusContext();
  
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">Package Registry</h1>
        <p className="hero__subtitle">
          Discover and install packages for your StelLang projects
        </p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/getting-started">
            Get Started
          </Link>
          <Link
            className="button button--outline button--lg"
            to="/docs/publishing">
            Publish Package
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Registry() {
  const [packages, setPackages] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');

  const fetchPackages = async (query = '') => {
    setLoading(true);
    setError(null);
    
    try {
      const url = query 
        ? `${REGISTRY_URL}/api/search?q=${encodeURIComponent(query)}`
        : `${REGISTRY_URL}/api/search?q=`;
      
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const data = await response.json();
      setPackages(data.packages || []);
    } catch (err) {
      console.error('Failed to fetch packages:', err);
      setError('Failed to load packages. Please try again later.');
      // Fallback to mock data for development
      setPackages([
        {
          name: 'example-http',
          version: '1.0.0',
          description: 'HTTP client library for StelLang',
          authors: ['stellang-team'],
          upload_date: new Date().toISOString(),
          size: 10240,
        },
        {
          name: 'example-json',
          version: '2.1.0',
          description: 'JSON parsing library for StelLang',
          authors: ['stellang-team'],
          upload_date: new Date().toISOString(),
          size: 15360,
        },
        {
          name: 'example-database',
          version: '0.5.0',
          description: 'Database connectivity library',
          authors: ['stellang-team'],
          upload_date: new Date().toISOString(),
          size: 20480,
        }
      ]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPackages();
  }, []);

  const handleSearch = (query) => {
    setSearchQuery(query);
    fetchPackages(query);
  };

  return (
    <Layout
      title="Package Registry"
      description="Discover and install packages for your StelLang projects">
      <RegistryHeader />
      <main className={styles.main}>
        <div className="container">
          <SearchBar onSearch={handleSearch} />
          
          {loading && (
            <div className={styles.loading}>
              <div className={styles.spinner}></div>
              <p>Loading packages...</p>
            </div>
          )}
          
          {error && (
            <div className={styles.error}>
              <p>{error}</p>
              <button onClick={() => fetchPackages(searchQuery)}>
                Try Again
              </button>
            </div>
          )}
          
          {!loading && !error && (
            <>
              <div className={styles.resultsHeader}>
                <h2>
                  {searchQuery ? `Search results for "${searchQuery}"` : 'All Packages'}
                </h2>
                <span className={styles.packageCount}>
                  {packages.length} package{packages.length !== 1 ? 's' : ''}
                </span>
              </div>
              
              {packages.length === 0 ? (
                <div className={styles.noResults}>
                  <p>No packages found.</p>
                  {searchQuery && (
                    <button onClick={() => handleSearch('')}>
                      View all packages
                    </button>
                  )}
                </div>
              ) : (
                <div className={styles.packageGrid}>
                  {packages.map((pkg, index) => (
                    <PackageCard key={`${pkg.name}-${pkg.version}-${index}`} package={pkg} />
                  ))}
                </div>
              )}
            </>
          )}
        </div>
      </main>
    </Layout>
  );
} 