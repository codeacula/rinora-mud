import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm';

@Entity('rooms')
export default class Room {
  constructor() {
    this.id = 0;
    this.area_id = 0;
    this.environment_id = 0;
    this.name = '';
    this.description = '';
  }

  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  area_id: number;

  @Column()
  environment_id: number;

  @Column()
  name: string;

  @Column()
  description: string;
}
