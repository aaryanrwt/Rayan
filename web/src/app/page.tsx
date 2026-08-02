"use client";

import { useEffect, useState } from 'react';

export default function Dashboard() {
  const [graphs, setGraphs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // In a real environment, this would hit http://localhost:3000/search
    // We mock the fetch for visual builder prototyping to prevent CORS/startup errors.
    setTimeout(() => {
      setGraphs([
        {
          id: "theprimeagen/neovim",
          nodes: [{ id: "neovim", kind: { Package: { name: "neovim" } } }]
        },
        {
          id: "rust-dev-minimal",
          nodes: [{ id: "rustup", kind: { Package: { name: "rustup" } } }]
        }
      ]);
      setLoading(false);
    }, 1000);
  }, []);

  return (
    <div style={{ padding: '2rem', fontFamily: 'sans-serif', backgroundColor: '#09090b', color: 'white', minHeight: '100vh' }}>
      <h1 style={{ fontSize: '3rem', fontWeight: 'bold' }}>Rayan Visual Builder</h1>
      <p style={{ marginTop: '1rem', color: '#a1a1aa' }}>Explore and visually compose Abstract Semantic Graphs (ASG).</p>
      
      {loading ? (
        <p style={{ marginTop: '2rem' }}>Loading global commons...</p>
      ) : (
        <div style={{ marginTop: '3rem', display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '2rem' }}>
          {graphs.map((graph, idx) => (
            <div key={idx} style={{ padding: '1.5rem', border: '1px solid #27272a', borderRadius: '8px', backgroundColor: '#18181b' }}>
              <h2 style={{ fontSize: '1.5rem', fontWeight: 'bold' }}>{graph.id}</h2>
              <div style={{ marginTop: '1rem', padding: '1rem', backgroundColor: '#27272a', borderRadius: '4px' }}>
                <pre style={{ fontSize: '0.8rem', color: '#10b981' }}>
                  {JSON.stringify(graph.nodes, null, 2)}
                </pre>
              </div>
              <button style={{ marginTop: '1rem', padding: '0.5rem 1rem', backgroundColor: '#facc15', color: 'black', border: 'none', borderRadius: '4px', fontWeight: 'bold' }}>
                Fork into Builder
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
