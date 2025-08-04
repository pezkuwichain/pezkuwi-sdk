import React from 'react';
import {
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  View,
  TouchableOpacity,
} from 'react-native';

const App = () => {
  return (
    <SafeAreaView style={styles.container}>
      <StatusBar barStyle="light-content" backgroundColor="#C8102E" />
      <View style={styles.header}>
        <Text style={styles.title}>🏛️ DKS</Text>
        <Text style={styles.subtitle}>Dijital Kurdistan State</Text>
      </View>
      
      <ScrollView style={styles.content}>
        <View style={styles.card}>
          <Text style={styles.cardTitle}>🆔 Digital Citizenship</Text>
          <Text style={styles.cardDesc}>Your digital identity in Kurdistan blockchain</Text>
        </View>
        
        <View style={styles.card}>
          <Text style={styles.cardTitle}>💰 HEZ Wallet</Text>
          <Text style={styles.cardDesc}>Native Kurdistan currency wallet</Text>
        </View>
        
        <View style={styles.card}>
          <Text style={styles.cardTitle}>🗳️ Parliament</Text>
          <Text style={styles.cardDesc}>Participate in digital governance</Text>
        </View>
        
        <View style={styles.card}>
          <Text style={styles.cardTitle}>🎓 Education Score</Text>
          <Text style={styles.cardDesc}>Perwerde educational achievements</Text>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFFFFF',
  },
  header: {
    backgroundColor: '#C8102E',
    padding: 20,
    alignItems: 'center',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#FFFFFF',
    marginBottom: 5,
  },
  subtitle: {
    fontSize: 16,
    color: '#FFFFFF',
    opacity: 0.9,
  },
  content: {
    flex: 1,
    padding: 20,
  },
  card: {
    backgroundColor: '#F8F9FA',
    padding: 20,
    marginBottom: 15,
    borderRadius: 10,
    borderLeftWidth: 4,
    borderLeftColor: '#00A550',
  },
  cardTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#2C3E50',
    marginBottom: 8,
  },
  cardDesc: {
    fontSize: 14,
    color: '#7F8C8D',
    lineHeight: 20,
  },
});

export default App;
