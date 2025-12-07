interface HomeProps {
  title: string
  message: string
}

export default function Home({ title, message }: HomeProps) {
  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h1>{title}</h1>
      <p>{message}</p>
      <p style={{ marginTop: '2rem', color: '#666' }}>
        Edit <code>frontend/src/pages/Home.tsx</code> to get started.
      </p>
    </div>
  )
}
